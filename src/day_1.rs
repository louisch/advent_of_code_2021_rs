/// Takes a string of whitespace separated integers and returns those integers in a Vec
pub fn parse_ints_to_vec(s: &str) -> Vec<u64> {
    return s.split_whitespace().map(|word| word.parse::<u64>().unwrap_or(0)).collect::<Vec<u64>>()
}

/// Day 1: Sonar Sweep (Part 1)
/// https://adventofcode.com/2021/day/1
///
/// Counts the number of times an integer b is higher than the integer a that immediately
/// precedes it inside the given vector v.
pub fn count_increased_measurements(v: &Vec<u64>) -> u64 {
    if v.len() <= 1 {
        return 0;
    }

    let mut count = 0;
    for i in 1..v.len() {
        if v[i - 1] < v[i] {
            count += 1;
        }
    }

    return count;
}

/// Day 1: Sonar Sweep (Part 2)
/// https://adventofcode.com/2021/day/1#part2
///
/// Similar to part 1, but now instead of the individual integers in v, count from
/// the 3 measurement sliding window.
pub fn count_3_measurement_sum_increased(v: &Vec<u64>) -> u64 {
    if v.len() <= 3 {
        return 0;
    }

    let mut count = 0;
    for i in 3..v.len() {
        if v[i - 3] < v[i] {
            count += 1;
        }
    }

    return count;
}


#[cfg(test)]
mod tests {
    use crate::day_1::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_ints_to_vec(""), []);
        assert_eq!(parse_ints_to_vec("0"), [0]);
        assert_eq!(parse_ints_to_vec("0 1"), [0, 1]);
        assert_eq!(parse_ints_to_vec("0\n1"), [0, 1]);
        assert_eq!(parse_ints_to_vec("199\n200\n208"), [199, 200, 208]);
    }

    #[test]
    fn test_count_part1() {
        assert_eq!(count_increased_measurements(&vec![]), 0);
        assert_eq!(count_increased_measurements(&vec![1]), 0);
        assert_eq!(count_increased_measurements(&vec![1, 2, 3]), 2);
        assert_eq!(count_increased_measurements(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7);
    }

    #[test]
    fn test_count_part2() {
        assert_eq!(count_3_measurement_sum_increased(&vec![]), 0);
        assert_eq!(count_3_measurement_sum_increased(&vec![1]), 0);
        assert_eq!(count_3_measurement_sum_increased(&vec![1, 2, 3]), 0);
        assert_eq!(count_3_measurement_sum_increased(&vec![1, 2, 3, 4]), 1);
        assert_eq!(count_3_measurement_sum_increased(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }
}
