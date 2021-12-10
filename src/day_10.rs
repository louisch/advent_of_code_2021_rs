use std::collections::HashMap;


enum SyntaxCheckResult {
    CorruptedChunk(usize, char, Option<char>),
    Incomplete(Vec<char>),
}

const OPEN_PAREN: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_PAREN: [char; 4] = [')', ']', '}', '>'];
const CORRUPTED_CHUNK_POINTS: [u64; 4] = [3, 57, 1197, 25137];
const AUTOCOMPLETE_POINTS: [u64; 4] = [1, 2, 3, 4];

fn assemble_table<T, U>(arr1: [T; 4], arr2: [U; 4]) -> HashMap<T, U>
    where T: Eq + std::hash::Hash + Copy {
    arr1.iter().map(|p| *p).zip(arr2).collect()
}

fn check_syntax(line: &String) -> SyntaxCheckResult {
    let parens_table: HashMap<char, char> = assemble_table(OPEN_PAREN, CLOSE_PAREN);
    let mut open_parens = vec![];
    for (i, paren) in line.chars().enumerate() {
        if parens_table.contains_key(&paren) {
            open_parens.push(paren);
        } else {
            let mut find_expected_parens = || {
                let last_open_paren = open_parens.pop()?;
                parens_table.get(&last_open_paren)
            };
            match find_expected_parens() {
                Some(expected_paren) if *expected_paren == paren => {},
                expected_paren_option => {
                    return SyntaxCheckResult::CorruptedChunk(i, paren, expected_paren_option.map(|c| *c));
                },
            }
        }
    }
    SyntaxCheckResult::Incomplete(open_parens)
}

fn get_corrupted_chunk_points(lines: &Vec<String>) -> u64 {
    let points_table: HashMap<char, u64> = assemble_table(CLOSE_PAREN, CORRUPTED_CHUNK_POINTS);
    let corrupted_chunks = find_corrupted_chunks(lines);
    corrupted_chunks.into_iter().fold(0, |acc, close_parens| acc + points_table.get(&close_parens).unwrap_or(&0))
}

fn find_corrupted_chunks(lines: &Vec<String>) -> Vec<char> {
    lines.into_iter().filter_map(|line| {
        let result = check_syntax(line);
        match result {
            SyntaxCheckResult::CorruptedChunk(_, paren, _) => Some(paren),
            _ => None,
        }
    }).collect()
}

pub fn part_1(lines: &Vec<String>) {
    let points = get_corrupted_chunk_points(lines);
    println!("Syntax Error Score (corrupted chunks): {}", points);
}


fn get_autocomplete_points(lines: &Vec<String>) -> u64 {
    let parens_table: HashMap<char, char> = assemble_table(OPEN_PAREN, CLOSE_PAREN);
    let points_table: HashMap<char, u64> = assemble_table(CLOSE_PAREN, AUTOCOMPLETE_POINTS);
    let mut all_scores: Vec<u64> = lines.into_iter().filter_map(|line| {
        let results = check_syntax(line);
        match results {
            SyntaxCheckResult::Incomplete(open_parens) => {
                let points = open_parens.iter().rev().filter_map(|parens| {
                    let close_parens = parens_table.get(parens)?;
                    let points = points_table.get(close_parens)?;
                    Some(points)
                }).fold(0, |acc, next| acc * 5 + next);
                Some(points)
            },
            _ => None,
        }
    }).collect();

    if all_scores.is_empty() {
        return 0;
    }

    all_scores.sort();
    all_scores[all_scores.len() / 2]
}

pub fn part_2(lines: &Vec<String>) {
    let points = get_autocomplete_points(lines);
    println!("Syntax Error Score (autocomplete): {}", points);
}


#[cfg(test)]
mod tests {
    use crate::day_10::*;

    const TEST_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn test_corrupted_chunk_points() {
        assert_eq!(get_corrupted_chunk_points(&get_test_input()), 26397);
    }

    #[test]
    fn test_autocomplete_points() {
        assert_eq!(get_autocomplete_points(&get_test_input()), 288957);
    }
}
