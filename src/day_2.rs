pub struct Position {
    pub horizontal: i64,
    pub depth: i64,
}

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Instruction {
    direction: Direction,
    distance: i64,
}

fn parse_direction(dir_str: &str) -> Option<Direction> {
    return match dir_str {
        "forward" => Some(Direction::Forward),
        "down" => Some(Direction::Down),
        "up" => Some(Direction::Up),
        _ => None,
    }
}

fn parse_instruction(instruction_str: &String) -> Option<Instruction> {
    let parts = instruction_str.trim().split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 {
        return None;
    }
    let direction = parse_direction(parts[0])?;
    let distance = parts[1].parse::<i64>().ok()?;
    return Some(Instruction {
        direction: direction,
        distance: distance,
    });
}

fn parse_instructions(instructions_str: &Vec<String>) -> Vec<Instruction> {
    return instructions_str.iter()
        .filter_map(|instr| parse_instruction(instr))
        .collect::<Vec<Instruction>>();
}

fn calculate_position_part1(course: &Vec<Instruction>) -> Position {
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;

    for instruction in course {
        match &instruction.direction {
            Direction::Forward => {
                horizontal += instruction.distance;
            },
            Direction::Down => {
                depth += instruction.distance;
            },
            Direction::Up => {
                depth -= instruction.distance;
            },
        }
    }

    return Position {
        horizontal: horizontal,
        depth: depth,
    }
}

fn calculate_position_part2(course: &Vec<Instruction>) -> Position {
    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for instruction in course {
        match &instruction.direction {
            Direction::Forward => {
                horizontal += instruction.distance;
                depth += aim * instruction.distance;
            },
            Direction::Down => {
                aim += instruction.distance;
            },
            Direction::Up => {
                aim -= instruction.distance;
            },
        }
    }

    return Position {
        horizontal: horizontal,
        depth: depth,
    }
}

pub fn parse_and_calculate_position_part1(instructions: &Vec<String>) -> Position {
    return calculate_position_part1(&parse_instructions(instructions));
}

pub fn parse_and_calculate_position_part2(instructions: &Vec<String>) -> Position {
    return calculate_position_part2(&parse_instructions(instructions));
}


#[cfg(test)]
mod tests {
    use crate::day_2::*;

    fn get_empty_input() -> Vec<String> {
        vec!["".to_string()]
    }
    fn get_input() -> Vec<String> {
        let input = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        input.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    fn assert_position(position: &Position, expected_horizontal: i64, expected_depth: i64) {
        assert_eq!(position.horizontal, expected_horizontal);
        assert_eq!(position.depth, expected_depth);
    }

    #[test]
    fn test_calculate_position_part1() {
        assert_position(&parse_and_calculate_position_part1(&get_empty_input()), 0, 0);
        assert_position(&parse_and_calculate_position_part1(&get_input()), 15, 10);
    }

    #[test]
    fn test_calculate_position_part2() {
        assert_position(&parse_and_calculate_position_part2(&get_empty_input()), 0, 0);
        assert_position(&parse_and_calculate_position_part2(&get_input()), 15, 60);
    }
}
