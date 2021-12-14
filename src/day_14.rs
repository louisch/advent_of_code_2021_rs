use std::collections::HashMap;


pub fn part_1(lines: &Vec<String>) {
    if let Some(stats) = stats_after_n_insertions(lines, 10) {
        println!("Stats: {:?}, Diff: {}", stats, stats.mce_count - stats.lce_count);
    } else {
        println!("Failed to find stats!");
    }
}

fn stats_after_n_insertions(lines: &Vec<String>, n: usize) -> Option<PolymerStats> {
    let (mut polymer, rules) = parse_input(lines);
    for _ in 0..n {
        polymer = insert_pairs_once(&polymer, &rules);
    }
    get_polymer_stats(&polymer)
}

pub fn part_2(lines: &Vec<String>) {
}


fn parse_input(lines: &Vec<String>) -> (String, HashMap<String, char>) {
    let polymer_template = lines[0].to_string();

    let mut rules: HashMap<String, char> = HashMap::new();
    for rule in lines[2..].iter() {
        let rule_parts = rule.split(" -> ").collect::<Vec<&str>>();
        if rule_parts.len() != 2 {
            continue;
        }
        let pair = rule_parts[0].to_string();
        if let Some(element) = rule_parts[1].chars().next() {
            rules.insert(pair, element);
        }
    }

    (polymer_template, rules)
}

fn insert_pairs_once(polymer: &str, insertion_rules: &HashMap<String, char>) -> String {
    let elements = polymer.chars().collect::<Vec<char>>();
    if elements.len() < 2 {
        return polymer.to_string();
    }

    let mut new_polymer = String::new();
    for (element1, element2) in elements[..elements.len()].iter().zip(elements[1..].iter()) {
        let pair = format!("{}{}", element1, element2);
        new_polymer.push(*element1);
        if let Some(new_element) = insertion_rules.get(&pair) {
            new_polymer.push(*new_element);
        }
    }
    new_polymer.push(elements[elements.len() - 1]);

    new_polymer
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

fn get_polymer_stats(polymer: &str) -> Option<PolymerStats> {
    let mut elements = polymer.chars().collect::<Vec<char>>();
    elements.sort();
    if elements.is_empty() {
        return None;
    }

    let mut highest_count = 0;
    let mut most_common_element = None;
    let mut lowest_count = u64::MAX;
    let mut least_common_element = None;
    let mut current_count = 0;
    let mut current_element = elements[0];

    for element in elements {
        if element != current_element {
            if current_count > highest_count {
                most_common_element = Some(current_element);
                highest_count = current_count;
            }
            if current_count < lowest_count {
                least_common_element = Some(current_element);
                lowest_count = current_count;
            }
            current_count = 1;
            current_element = element;
            continue;
        }
        current_count += 1;
    }
    if current_count > highest_count {
        most_common_element = Some(current_element);
        highest_count = current_count;
    }
    if current_count < lowest_count {
        least_common_element = Some(current_element);
        lowest_count = current_count;
    }

    let mce = most_common_element?;
    let lce = least_common_element?;

    Some(PolymerStats::new(mce, highest_count, lce, lowest_count))
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
        assert_eq!(stats_after_n_insertions(&get_test_input(TEST_INPUT_1), 40), Some(PolymerStats::new('B', 1749, 'H', 161)));
    }
}
