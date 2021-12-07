mod utils;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

use std::io::{self, Stdin};
use crate::utils::*;


fn day_1(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let s = parse_lines(stdin)?;
    let parsed = parse_ints_to_vec(&s);
    let count = match part {
        1 => Ok(day_1::count_increased_measurements(&parsed)),
        2 => Ok(day_1::count_3_measurement_sum_increased(&parsed)),
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }?;
    println!("Count: {}", count);
    Ok(())
}

fn day_2(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let s = parse_lines(stdin)?;
    let position = match part {
        1 => Ok(day_2::parse_and_calculate_position_part1(&s)),
        2 => Ok(day_2::parse_and_calculate_position_part2(&s)),
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }?;
    println!("Position: h {} d {}, Multiplied: {}", position.horizontal, position.depth, position.horizontal * position.depth);
    Ok(())
}

fn day_3(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let lines = parse_lines(stdin)?;
    match part {
        1 => {
            let (gamma_rate_vec, epsilon_rate_vec) = day_3::get_gamma_and_epsilon_rates(&lines);
            let gamma_rate_str = gamma_rate_vec.iter().collect::<String>();
            let epsilon_rate_str = epsilon_rate_vec.iter().collect::<String>();
            let gamma_rate = i64::from_str_radix(&gamma_rate_str, 2).unwrap();
            let epsilon_rate = i64::from_str_radix(&epsilon_rate_str, 2).unwrap();
            println!("Gamma Rate: {}, Epsilon Rate: {}, Multiplied: {}", gamma_rate_str, epsilon_rate_str, gamma_rate * epsilon_rate);
            Ok(())
        },
        2 => {
            let (o2gen_rate, co2scrubber_rate) = day_3::get_o2gen_and_co2scrubber_rates(&lines);
            let o2gen_rate_i64 = i64::from_str_radix(&o2gen_rate, 2).unwrap();
            let co2scrubber_rate_i64 = i64::from_str_radix(&co2scrubber_rate, 2).unwrap();
            println!("O2 generator rate: {}, CO2 scrubber rate: {}, Multiplied: {}", o2gen_rate, co2scrubber_rate, o2gen_rate_i64 * co2scrubber_rate_i64);
            Ok(())
        },
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }
}

fn day_4(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let mut s = parse_lines(stdin)?;
    match part {
        1 => {
            let (winning_number, winning_board) = day_4::find_winning_bingo_board(&mut s).ok_or(anyhow::anyhow!("No board found!"))?;
            let board_as_numbers = winning_board.map(|square| square.number);
            println!("Winning Board:\n{:?}", board_as_numbers);
            let score = day_4::get_score(winning_number, &winning_board);
            println!("Winning Number: {}, Score: {}", winning_number, score);
            Ok(())
        },
        2 => {
            let (number, board) = day_4::find_last_to_win_bingo_board(&mut s).ok_or(anyhow::anyhow!("No board found!"))?;
            let board_as_numbers = board.map(|square| square.number);
            println!("Last Board:\n{:?}", board_as_numbers);
            let score = day_4::get_score(number, &board);
            println!("Winning Number: {}, Score: {}", number, score);
            Ok(())
        }
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }
}

fn day_5(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let lines = parse_lines(stdin)?;
    match part {
        1 => {
            let count = day_5::count_overlapping_orthogonal_ventlines(&lines);
            println!("Overlapping points: {}", count);
            Ok(())
        },
        2 => {
            let count = day_5::count_overlapping_ventlines(&lines);
            println!("Overlapping points: {}", count);
            Ok(())
        },
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }
}

fn day_6(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let lines = parse_lines(stdin)?;
    match part {
        1 => {
            let count = day_6::count_lanternfish(&lines, 80);
            println!("Number of Lanternfish: {}", count);
            Ok(())
        },
        2 => {
            let count = day_6::count_lanternfish(&lines, 256);
            println!("Number of Lanternfish: {}", count);
            Ok(())
        },
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }
}

fn day_7(stdin: &Stdin) -> anyhow::Result<()> {
    println!("Choose Part:");
    let part = read_number(&stdin)?;
    let lines = parse_lines(stdin)?;
    match part {
        1 => {
            let (optimum, fuel) = day_7::find_optimum_constant(&lines);
            println!("Optimal Position: {}, Total Fuel Cost: {}", optimum, fuel);
            Ok(())
        },
        2 => {
            let (optimum, fuel) = day_7::find_optimum_triangular(&lines);
            println!("Optimal Position: {}, Total Fuel Cost: {}", optimum, fuel);
            Ok(())
        },
        _ => Err(anyhow::anyhow!("Unknown part: {}", part)),
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    println!("Choose Day:");
    let day = read_number(&stdin)?;
    match day {
        1 => day_1(&stdin),
        2 => day_2(&stdin),
        3 => day_3(&stdin),
        4 => day_4(&stdin),
        5 => day_5(&stdin),
        6 => day_6(&stdin),
        7 => day_7(&stdin),
        _ => Err(anyhow::anyhow!("Unknown Day: {}", day)),
    }
}
