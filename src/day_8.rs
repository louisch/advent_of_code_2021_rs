use std::collections::{HashSet,HashMap};
use num_traits::pow;


/// Part 1 is fairly simple; we just go through the lines and count the number of segments
pub fn part_1(lines: &Vec<String>) {
    println!("1,4,7 and 8 appear {} times", count_unique_digits(lines));
}

fn count_unique_digits(lines: &Vec<String>) -> i64 {
    let mut count = 0;
    let unique_segment_lens: HashSet<usize> = vec![2, 3, 4, 7].into_iter().collect(); // 1, 7, 4, and 8
    for line in lines {
        let parts = line.split(" | ").collect::<Vec<&str>>();
        if parts.len() < 2 {
            continue;
        }
        count += parts[1].split_whitespace().filter(|digit_str| unique_segment_lens.contains(&digit_str.len())).count() as i64;
    }
    count
}


/// The strategy for part 2 is to first deduce a "digit map", being a map from the string of
/// present segments in a given digit (this string must be sorted) to the digit those segments
/// represent in that particular line, then to use this digit map to translate the four digits
/// in the right hand side.
pub fn part_2(lines: &Vec<String>) {
    let sum = sum_output_values(lines);
    println!("Sum of Output Values: {}", sum);
}

/// Utility function for `deduce_digit_map`
fn segment_diff_check(strings: Vec<String>, diff_string: String, target_length: i32) -> Option<(String, Vec<String>)> {
    let (s1_vec, others) = strings.into_iter().partition::<Vec<String>, _>(|s| {
        s.chars().into_iter().filter(|c1| !diff_string.contains(|c2: char| *c1 == c2)).count() == target_length as usize
    });
    let s1 = s1_vec.into_iter().next()?;
    Some((s1, others))
}

/// Our strategy to deduce the "digit map" for a particular set of ten signal patterns is to
/// 1. Find the four patterns which represent the four numbers that have a unique number of
///    segments. (1, 4, 7, and 8)
/// 2. Separate the remaining digits into two sets based on how many segments they have.
///    (2, 3, 5 and 0, 6, 9)
/// 3. "Subtract" segments from the digits in each unknown set with the digits we have already
///    found, in such a way that the remaining number of segments identifies one of the digits
///    in that set. Rinse and repeat.
/// For example, to distinguish 2, we exploit the fact that 2 is the only one out of 2, 3, and 5
/// that has an e segment, which the pattern for 4 does not have. This means taking the set
/// difference with 4 will leave the 2 with three segments, while 3 and 5 will have two, and we
/// can simply partition based on whether there are three segments left or not.
fn deduce_digit_map(signal_patterns_str: String) -> Option<HashMap<String, i32>> {
    let signal_patterns: Vec<String> = signal_patterns_str
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let pattern_1 = signal_patterns.iter().find(|pattern| pattern.len() == 2)?.to_string(); // contains the c and f segments
    let pattern_4 = signal_patterns.iter().find(|pattern| pattern.len() == 4)?.to_string(); // contains the b, c, d, and f segments
    let pattern_7 = signal_patterns.iter().find(|pattern| pattern.len() == 3)?.to_string(); // contains the a, c, and f segments
    let pattern_8 = signal_patterns.iter().find(|pattern| pattern.len() == 7)?.to_string(); // contains the a, c, and f segments
    let patterns_235: Vec<String> = signal_patterns.iter().filter(|pattern| pattern.len() == 5).map(|pattern| pattern.clone()).collect();
    let patterns_069: Vec<String> = signal_patterns.iter().filter(|pattern| pattern.len() == 6).map(|pattern| pattern.clone()).collect();

    let (pattern_2, patterns_35) = segment_diff_check(patterns_235, pattern_4.clone(), 3)?;
    let (pattern_3, pattern_5_vec) = segment_diff_check(patterns_35, pattern_2.clone(), 1)?;
    let pattern_5 = pattern_5_vec.into_iter().next()?;

    let (pattern_6, patterns_09) = segment_diff_check(patterns_069, pattern_7.clone(), 4)?;
    let (pattern_0, pattern_9_vec) = segment_diff_check(patterns_09, pattern_4.clone(), 3)?;
    let pattern_9 = pattern_9_vec.into_iter().next()?;

    let patterns: HashMap<String, i32> = [
        pattern_0,
        pattern_1,
        pattern_2,
        pattern_3,
        pattern_4,
        pattern_5,
        pattern_6,
        pattern_7,
        pattern_8,
        pattern_9,
    ].iter()
        .enumerate()
        .map(|(i, pattern)| {
            let mut pattern_vec: Vec<char> = pattern.chars().collect();
            pattern_vec.sort();
            (pattern_vec.into_iter().collect(), i as i32)
        })
    .collect();
    Some(patterns)
}

fn sum_output_values(lines: &Vec<String>) -> i64 {
    lines.iter().filter_map(|line| {
        let separator_index = line.find(" | ")?;
        let (signal_pattern, output_value_digits) = line.split_at(separator_index);

        let digit_map = deduce_digit_map(signal_pattern.to_string())?;
        let output_value = output_value_digits
            .split_whitespace()
            .filter_map(|digit_str| {
                let mut digit_str_sorted: Vec<char> = digit_str.chars().collect();
                digit_str_sorted.sort();
                digit_map.get(&digit_str_sorted.into_iter().collect::<String>())
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (index, digit)| acc + (pow(10, index) * digit) as i64);
        Some(output_value)
    }).sum()
}


#[cfg(test)]
mod tests {
    use crate::day_8::*;

    const TEST_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_count_unique_digits() {
        let test_input = TEST_INPUT.split('\n').map(str::to_string).collect::<Vec<String>>();
        assert_eq!(count_unique_digits(&test_input), 26);
    }

    #[test]
    fn test_sum_output_values() {
        let test_input = TEST_INPUT.split('\n').map(str::to_string).collect::<Vec<String>>();
        assert_eq!(sum_output_values(&test_input), 61229);
    }
}
