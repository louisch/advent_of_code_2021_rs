use std::collections::HashMap;


pub fn part_1(lines: &Vec<String>) {
    if let Some(stats) = stats_after_n_insertions(lines, 10) {
        println!("Stats: {:?}, Diff: {}", stats, stats.mce_count - stats.lce_count);
    } else {
        println!("Failed to find stats!");
    }
}

pub fn part_2(lines: &Vec<String>) {
    if let Some(stats) = stats_after_n_insertions(lines, 40) {
        println!("Stats: {:?}, Diff: {}", stats, stats.mce_count - stats.lce_count);
    } else {
        println!("Failed to find stats!");
    }
}

fn stats_after_n_insertions(lines: &Vec<String>, n: usize) -> Option<PolymerStats> {
    let (mut counts, rules, first_element, last_element) = parse_input(lines);
    for _ in 0..n {
        insert_pairs_once(&mut counts, &rules);
    }
    get_polymer_stats(&counts, first_element, last_element)
}


fn parse_input(lines: &Vec<String>) -> (HashMap<(char, char), usize>, HashMap<(char, char), char>, char, char) {
    let polymer_template = lines[0].chars().collect::<Vec<char>>();
    let first_element = polymer_template[0];
    let last_element = polymer_template[polymer_template.len() - 1];
    let mut counts: HashMap<(char, char), usize> = HashMap::new();
    for (element1, element2) in polymer_template[..polymer_template.len()].iter().zip(polymer_template[1..].iter()) {
        *(counts.entry((*element1, *element2)).or_insert(0)) += 1;
    }

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    for rule in lines[2..].iter() {
        let rule_parts = rule.split(" -> ").collect::<Vec<&str>>();
        if rule_parts.len() != 2 {
            continue;
        }
        let pair = rule_parts[0].chars().collect::<Vec<char>>();
        if let Some(element) = rule_parts[1].chars().next() {
            rules.insert((pair[0], pair[1]), element);
        }
    }

    (counts, rules, first_element, last_element)
}

fn insert_pairs_once(counts: &mut HashMap<(char, char), usize>, insertion_rules: &HashMap<(char, char), char>) {
    let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();
    for (pair, new_element) in insertion_rules {
        if let Some(count) = counts.remove(pair) {
            let (element1, element2) = pair;
            let left_pair = (*element1, *new_element);
            let right_pair = (*new_element, *element2);
            *(new_pairs.entry(left_pair).or_insert(0)) += count;
            *(new_pairs.entry(right_pair).or_insert(0)) += count;
        }
    }
    for (new_pair, count) in new_pairs {
        *(counts.entry(new_pair).or_insert(0)) += count;
    }
}

#[derive(Debug, PartialEq)]
struct PolymerStats {
    most_common_element: char,
    mce_count: u64,
    least_common_element: char,
    lce_count: u64,
}

impl PolymerStats {
    fn new(mce: char, mce_count: u64, lce: char, lce_count: u64) -> Self {
        Self {
            most_common_element: mce,
            mce_count: mce_count,
            least_common_element: lce,
            lce_count: lce_count,
        }
    }
}

fn get_polymer_stats(pair_counts: &HashMap<(char, char), usize>, first_element: char, last_element: char) -> Option<PolymerStats> {
    let mut element_counts: HashMap<char, usize> = HashMap::new();

    for ((el1, el2), count) in pair_counts {
        *(element_counts.entry(*el1).or_insert(0)) += count;
        *(element_counts.entry(*el2).or_insert(0)) += count;
    }

    let mut counts_vec = element_counts.into_iter().map(|(el, count)| {
        let mut true_count = count;
        if el == first_element {
            true_count += 1;
        }
        if el == last_element {
            true_count += 1;
        }
        true_count /= 2;
        (el, true_count)
    }).collect::<Vec<(char, usize)>>();
    counts_vec.sort_by(|(_, count1), (_, count2)| {
        count1.partial_cmp(count2).unwrap()
    });
    let (mce, mce_count) = counts_vec.last()?;
    let (lce, lce_count) = counts_vec.first()?;

    Some(PolymerStats::new(*mce, *mce_count as u64, *lce, *lce_count as u64))
}


#[cfg(test)]
mod tests {
    use crate::day_14::*;

    const TEST_INPUT_1: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(stats_after_n_insertions(&get_test_input(TEST_INPUT_1), 10), Some(PolymerStats::new('B', 1749, 'H', 161)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(stats_after_n_insertions(&get_test_input(TEST_INPUT_1), 40), Some(PolymerStats::new('B', 2192039569602, 'H', 3849876073)));
    }
}
