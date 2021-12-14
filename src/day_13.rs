use std::collections::HashSet;


pub fn part_1(lines: &Vec<String>) {
    let count = perform_first_fold(lines);
    println!("Visible dots: {}", count);
}

pub fn part_2(lines: &Vec<String>) {
    let paper = perform_all_folds(lines);
    let success = visualize_paper(&paper);
    if success.is_none() {
        println!("Failed to visualize paper!");
    }
}


#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Left,
}

#[derive(Debug)]
struct FoldInstruction {
    direction: Direction,
    position: usize,
}

fn visualize_paper(paper: &HashSet<(usize, usize)>) -> Option<()> {
    let max_x = paper.iter().map(|(x, _)| *x).max()?;
    let max_y = paper.iter().map(|(_, y)| *y).max()?;
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            print!("{}", if paper.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }
    Some(())
}

fn perform_first_fold(lines: &Vec<String>) -> u64 {
    let (paper, instructions) = parse_input(lines);
    perform_instructions_n(&paper.into_iter().collect::<HashSet<(usize, usize)>>(), &instructions, 1).len() as u64
}

fn perform_all_folds(lines: &Vec<String>) -> HashSet<(usize, usize)> {
    let (paper, instructions) = parse_input(lines);
    perform_instructions_n(&paper.into_iter().collect::<HashSet<(usize, usize)>>(), &instructions, usize::MAX)
}

fn parse_input(lines: &Vec<String>) -> (Vec<(usize, usize)>, Vec<FoldInstruction>) {
    let mut paper = vec![];
    let mut paper_done = false;
    let mut instructions = vec![];

    for line in lines {
        let trimmed = line.trim();

        // Skip any empty lines at the beginning
        if paper.is_empty() && trimmed.is_empty() {
            continue;
        }

        if !paper_done && !trimmed.is_empty() {
            let coords: Vec<usize> = trimmed.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            paper.push((coords[0], coords[1]));
            continue;
        }

        // Skip empty line between paper and instructions
        if instructions.is_empty() && trimmed.is_empty() {
            paper_done = true;
            continue;
        }

        if !trimmed.is_empty() {
            let separator_index = trimmed.find('=').unwrap();
            let (instruction_1, instruction_2) = trimmed.split_at(separator_index);
            let direction = if instruction_1.get(instruction_1.len() - 1..instruction_1.len()).unwrap() == "y" { Direction::Up } else { Direction::Left };
            let position = instruction_2[1..].parse::<usize>().unwrap();
            instructions.push(FoldInstruction { direction: direction, position: position });
        }
    }

    (paper, instructions)
}

fn perform_instructions_n(paper: &HashSet<(usize, usize)>, instructions: &Vec<FoldInstruction>, n: usize) -> HashSet<(usize, usize)> {
    let mut current_paper = paper.clone();
    for instruction in instructions.iter().take(n) {
        let mut new_paper = HashSet::new();
        if instruction.direction == Direction::Up {
            for (x, y) in current_paper.iter() {
                if y <= &instruction.position {
                    new_paper.insert((*x, *y));
                } else {
                    new_paper.insert((*x, (instruction.position * 2 - y) as usize));
                }
            }
        } else {
            for (x, y) in current_paper.iter() {
                if x <= &instruction.position {
                    new_paper.insert((*x, *y));
                } else {
                    new_paper.insert(((instruction.position * 2 - x) as usize, *y));
                }
            }
        }
        current_paper = new_paper;
    }
    current_paper
}


#[cfg(test)]
mod tests {
    use crate::day_13::*;

    const TEST_INPUT_1: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(perform_first_fold(&get_test_input(TEST_INPUT_1)), 17);
    }
}
