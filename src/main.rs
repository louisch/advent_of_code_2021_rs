mod day_1;
mod day_2;
mod day_3;

use std::io::{self, Stdin, BufRead};


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
    if buffer[buffer.len() - 1] == b'-' {
        buffer.pop();
    }
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
                    let count = day_1::count_increased_measurements(&parsed);
                    println!("Count: {}", count);
                },
                2 => {
                    let count = day_1::count_3_measurement_sum_increased(&parsed);
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
                    let position = day_2::parse_and_calculate_position_part1(&s);
                    println!("Position: h {} d {}, Multiplied: {}", position.horizontal, position.depth, position.horizontal * position.depth);
                },
                2 => {
                    let position = day_2::parse_and_calculate_position_part2(&s);
                    println!("Position: h {} d {}, Multiplied: {}", position.horizontal, position.depth, position.horizontal * position.depth);
                },
                _ => println!("Unknown part: {}", part),
            }
        },
        Err(e) => panic!("Failed to read part: {}", e),
    }
}

fn day_3(stdin: &Stdin) {
    println!("Choose Part:");
    let part_result = read_number(&stdin);
    match part_result {
        Ok(part) => {
            let s = parse_lines(stdin);
            let numbers = s.split_whitespace().collect::<Vec<&str>>();
            match part {
                1 => {
                    let (gamma_rate_vec, epsilon_rate_vec) = day_3::get_gamma_and_epsilon_rates(&numbers);
                    let gamma_rate_str = gamma_rate_vec.iter().collect::<String>();
                    let epsilon_rate_str = epsilon_rate_vec.iter().collect::<String>();
                    let gamma_rate = i64::from_str_radix(&gamma_rate_str, 2).unwrap();
                    let epsilon_rate = i64::from_str_radix(&epsilon_rate_str, 2).unwrap();
                    println!("Gamma Rate: {}, Epsilon Rate: {}, Multiplied: {}", gamma_rate_str, epsilon_rate_str, gamma_rate * epsilon_rate);
                },
                2 => {
                    let (o2gen_rate, co2scrubber_rate) = day_3::get_o2gen_and_co2scrubber_rates(&numbers);
                    let o2gen_rate_i64 = i64::from_str_radix(&o2gen_rate, 2).unwrap();
                    let co2scrubber_rate_i64 = i64::from_str_radix(&co2scrubber_rate, 2).unwrap();
                    println!("O2 generator rate: {}, CO2 scrubber rate: {}, Multiplied: {}", o2gen_rate, co2scrubber_rate, o2gen_rate_i64 * co2scrubber_rate_i64);
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
                3 => day_3(&stdin),
                _ => println!("Unknown Day: {}", day),
            }
        },
        Err(e) => panic!("Failed to read day: {}", e),
    }
}
