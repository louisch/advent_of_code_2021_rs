use std::io::{Stdin, BufRead};

/// Takes a string of whitespace separated integers and returns those integers in a Vec
pub fn parse_ints_to_vec(lines: &Vec<String>) -> Vec<u64> {
    return lines.iter().filter_map(|word| word.parse::<u64>().ok()).collect::<Vec<u64>>()
}

pub fn parse_lines(stdin: &Stdin) -> anyhow::Result<Vec<String>> {
    println!("Enter Input (separate numbers with whitespace, end sequence by entering the character '+' or an EOF character (CTRL-D on Unix, CTRL-Z on Windows):");

    let mut buffer = vec![];
    stdin.lock().read_until(b'+', &mut buffer)?;
    if buffer[buffer.len() - 1] == b'+' {
        buffer.pop();
    }
    let s = String::from_utf8(buffer)?;
    Ok(s.split('\n').map(str::to_string).collect::<Vec<String>>())
}


#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn test_parse() {
        let empty = vec!["".to_string()];
        let one = vec!["0".to_string()];
        let two = vec!["0", "1"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let three = vec!["199", "200", "208"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(parse_ints_to_vec(&empty), []);
        assert_eq!(parse_ints_to_vec(&one), [0]);
        assert_eq!(parse_ints_to_vec(&two), [0, 1]);
        assert_eq!(parse_ints_to_vec(&three), [199, 200, 208]);
    }
}
