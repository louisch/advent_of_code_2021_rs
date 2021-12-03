mod day_1;
mod day_2;

use std::io::{self, Stdin, BufRead};
use crate::day_1::*;
use crate::day_2::*;


/// Takes a string of whitespace separated integers and returns those integers in a Vec
fn parse_ints_to_vec(s: &str) -> Vec<u64> {
    return s.split_whitespace().map(|word| word.parse::<u64>().unwrap_or(0)).collect::<Vec<u64>>()
}

fn parse_lines(stdin: &Stdin) -> String {
    println!("Enter Input (separate numbers with whitespace, end sequence by entering the character '-' or an EOF character (CTRL-D on Unix, CTRL-Z on Windows):");

    let mut buffer = vec![];
    match stdin.lock().read_until(b'-', &mut buffer) {
        Ok(_) => {},
        Err(e) => panic!("Failed to read input: {}", e),
    };
    let s = match String::from_utf8(buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 string: {}", e),
    };

    return s;
}

fn read_number(stdin: &Stdin) -> anyhow::Result<u32> {
    let mut line = String::new();
    let _ = stdin.lock().read_line(&mut line)?;
    let number = line.trim().parse::<u32>()?;
    return Ok(number);
}


#[cfg(test)]
mod tests {
    use crate::parse_ints_to_vec;
    #[test]
    fn test_parse() {
        assert_eq!(parse_ints_to_vec(""), []);
        assert_eq!(parse_ints_to_vec("0"), [0]);
        assert_eq!(parse_ints_to_vec("0 1"), [0, 1]);
        assert_eq!(parse_ints_to_vec("0\n1"), [0, 1]);
        assert_eq!(parse_ints_to_vec("199\n200\n208"), [199, 200, 208]);
    }
}


fn day_1(stdin: &Stdin) {
    println!("Choose Part:");
    let part_result = read_number(&stdin);
    match part_result {
        Ok(part) => {
            let s = parse_lines(stdin);
            let parsed = parse_ints_to_vec(&s);
            match part {
                1 => {
                    let count = count_increased_measurements(&parsed);
                    println!("Count: {}", count);
                },
                2 => {
                    let count = count_3_measurement_sum_increased(&parsed);
                    println!("Count: {}", count);
                },
                _ => println!("Unknown part: {}", part),
            }

        },
        Err(e) => panic!("Failed to read part: {}", e),
    }
}

fn day_2(stdin: &Stdin) {
    println!("Choose Part:");
    let part_result = read_number(&stdin);
    match part_result {
        Ok(part) => {
            let s = parse_lines(stdin);
            match part {
                1 => {
                    let position = parse_and_calculate_position_part1(&s);
                    println!("Position: h {} d {}, Multiplied: {}", position.horizontal, position.depth, position.horizontal * position.depth);
                },
                2 => {
                    let position = parse_and_calculate_position_part2(&s);
                    println!("Position: h {} d {}, Multiplied: {}", position.horizontal, position.depth, position.horizontal * position.depth);
                },
                _ => println!("Unknown part: {}", part),
            }
        },
        Err(e) => panic!("Failed to read part: {}", e),
    }
}

fn main() {
    let stdin = io::stdin();

    println!("Choose Day:");
    let day_result = read_number(&stdin);
    match day_result {
        Ok(day) => {
            match day {
                1 => day_1(&stdin),
                2 => day_2(&stdin),
                _ => println!("Unknown Day: {}", day),
            }
        },
        Err(e) => panic!("Failed to read day: {}", e),
    }
}
