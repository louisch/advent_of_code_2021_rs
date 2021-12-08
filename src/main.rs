mod utils;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use crate::utils::*;


type AOCPartFn = fn(&Vec<String>) -> ();

fn day_3_part_1(lines: &Vec<String>) {
    let (gamma_rate_vec, epsilon_rate_vec) = day_3::get_gamma_and_epsilon_rates(&lines);
    let gamma_rate_str = gamma_rate_vec.iter().collect::<String>();
    let epsilon_rate_str = epsilon_rate_vec.iter().collect::<String>();
    let gamma_rate = i64::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = i64::from_str_radix(&epsilon_rate_str, 2).unwrap();
    println!("Gamma Rate: {}, Epsilon Rate: {}, Multiplied: {}", gamma_rate_str, epsilon_rate_str, gamma_rate * epsilon_rate);
}
fn day_3_part_2(lines: &Vec<String>) {
    let (o2gen_rate, co2scrubber_rate) = day_3::get_o2gen_and_co2scrubber_rates(&lines);
    let o2gen_rate_i64 = i64::from_str_radix(&o2gen_rate, 2).unwrap();
    let co2scrubber_rate_i64 = i64::from_str_radix(&co2scrubber_rate, 2).unwrap();
    println!("O2 generator rate: {}, CO2 scrubber rate: {}, Multiplied: {}", o2gen_rate, co2scrubber_rate, o2gen_rate_i64 * co2scrubber_rate_i64);
}

fn day_4_part_1(lines: &Vec<String>) {
    let mut s = lines.clone();
    let (winning_number, winning_board) = day_4::find_winning_bingo_board(&mut s).unwrap();
    let board_as_numbers = winning_board.map(|square| square.number);
    println!("Winning Board:\n{:?}", board_as_numbers);
    let score = day_4::get_score(winning_number, &winning_board);
    println!("Winning Number: {}, Score: {}", winning_number, score);
}
fn day_4_part_2(lines: &Vec<String>) {
    let mut s = lines.clone();
    let (number, board) = day_4::find_last_to_win_bingo_board(&mut s).unwrap();
    let board_as_numbers = board.map(|square| square.number);
    println!("Last Board:\n{:?}", board_as_numbers);
    let score = day_4::get_score(number, &board);
    println!("Winning Number: {}, Score: {}", number, score);
}

fn day_5_part_1(lines: &Vec<String>) {
    let count = day_5::count_overlapping_orthogonal_ventlines(&lines);
    println!("Overlapping points: {}", count);
}
fn day_5_part_2(lines: &Vec<String>) {
    let count = day_5::count_overlapping_ventlines(&lines);
    println!("Overlapping points: {}", count);
}

fn day_6_part_1(lines: &Vec<String>) {
    let count = day_6::count_lanternfish(&lines, 80);
    println!("Number of Lanternfish: {}", count);
}
fn day_6_part_2(lines: &Vec<String>) {
    let count = day_6::count_lanternfish(&lines, 256);
    println!("Number of Lanternfish: {}", count);
}

fn day_7_part_1(lines: &Vec<String>) {
    let (optimum, fuel) = day_7::find_optimum_constant(&lines);
    println!("Optimal Position: {}, Total Fuel Cost: {}", optimum, fuel);
}
fn day_7_part_2(lines: &Vec<String>) {
    let (optimum, fuel) = day_7::find_optimum_triangular(&lines);
    println!("Optimal Position: {}, Total Fuel Cost: {}", optimum, fuel);
}

fn main() -> anyhow::Result<()> {
    let mut map: HashMap<String, AOCPartFn> = HashMap::new();
    map.insert("1:1".to_string(), day_1::part_1);
    map.insert("1:2".to_string(), day_1::part_2);
    map.insert("2:1".to_string(), day_2::part_1);
    map.insert("2:2".to_string(), day_2::part_2);
    map.insert("3:1".to_string(), day_3_part_1);
    map.insert("3:2".to_string(), day_3_part_2);
    map.insert("4:1".to_string(), day_4_part_1);
    map.insert("4:2".to_string(), day_4_part_2);
    map.insert("5:1".to_string(), day_5_part_1);
    map.insert("5:2".to_string(), day_5_part_2);
    map.insert("6:1".to_string(), day_6_part_1);
    map.insert("6:2".to_string(), day_6_part_2);
    map.insert("7:1".to_string(), day_7_part_1);
    map.insert("7:2".to_string(), day_7_part_2);
    map.insert("8:1".to_string(), day_8::part_1);
    map.insert("8:2".to_string(), day_8::part_2);

    let stdin = io::stdin();
    let stdout = io::stdout();

    print!("Choose Day: ");
    stdout.lock().flush().unwrap();
    let mut key = String::new();
    let _ = stdin.lock().read_line(&mut key)?;
    key.remove(key.len() - 1);
    key.push(':');
    print!("Choose Part: ");
    stdout.lock().flush().unwrap();
    let _ = stdin.lock().read_line(&mut key)?;
    key.remove(key.len() - 1);

    let func_option = map.get(&key);
    if let Some(func) = func_option {
        let lines = parse_lines(&stdin)?;
        func(&lines);
    }
    Ok(())
}
