mod day_1;

use crate::day_1::{parse_ints_to_vec, count_3_measurement_sum_increased};
use std::io::{self, BufRead};

fn main() {
    println!("Enter Input (separate numbers with whitespace, end sequence by entering the character '-' or an EOF character (CTRL-D on Unix, CTRL-Z on Windows):");
    let stdin = io::stdin();

    let mut buffer = vec![];
    match stdin.lock().read_until(b'-', &mut buffer) {
        Ok(_) => {},
        Err(e) => panic!("Failed to read input: {}", e),
    };
    let s = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 string: {}", e),
    };

    let parsed = parse_ints_to_vec(s);
    let count = count_3_measurement_sum_increased(&parsed);

    println!("Count: {}", count);
}
