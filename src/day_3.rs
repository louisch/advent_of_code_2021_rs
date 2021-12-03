fn get_gamma_rate(numbers: &Vec<&str>) -> Vec<char> {
    if numbers.len() == 0 {
        return vec![];
    }

    let mut bits = vec![];
    for j in 0..numbers[0].len() {
        let mut zero_count = 0;
        let mut one_count = 0;
        for number in numbers {
            if j > number.len() {
                panic!("Number {} is larger than the first number in the sequence", number);
            }
            let bit = number.as_bytes()[j];
            if bit == b'0' {
                zero_count += 1
            } else if bit == b'1' {
                one_count += 1
            } else {
                panic!("Number {} contains the character {} which is not a 0 or 1", number, bit);
            }
        }
        bits.push(if one_count >= zero_count { '1' } else { '0' });
    }
    return bits;
}

pub fn get_gamma_and_epsilon_rates(numbers: &Vec<&str>) -> (Vec<char>, Vec<char>) {
    let gamma_rate = get_gamma_rate(numbers);
    let mut epsilon_rate = gamma_rate.clone();
    for i in 0..epsilon_rate.len() {
        epsilon_rate[i] = if epsilon_rate[i] == '0' { '1' } else { '0' };
    }
    return (gamma_rate, epsilon_rate);
}

pub fn get_o2gen_and_co2scrubber_rates<'a>(numbers: &Vec<&'a str>) -> (&'a str, &'a str) {
    if numbers.len() == 0 {
        panic!("Doesn't work for zero length vectors!");
    }

    let mut o2gen_numbers = numbers.clone();
    if o2gen_numbers.len() > 1 {
        for i in 0..o2gen_numbers[0].len() {
            if o2gen_numbers.len() <= 1 {
                break;
            }
            let gamma_rate = get_gamma_rate(&o2gen_numbers);
            o2gen_numbers = o2gen_numbers.into_iter()
                .filter(|o2gen_number| o2gen_number.as_bytes()[i] as char == gamma_rate[i])
                .collect::<Vec<&str>>();
        }
    }

    let mut co2scrubber_numbers = numbers.clone();
    if co2scrubber_numbers.len() > 1 {
        for i in 0..co2scrubber_numbers[0].len() {
            if co2scrubber_numbers.len() <= 1 {
                break;
            }
            let (_, epsilon_rate) = get_gamma_and_epsilon_rates(&co2scrubber_numbers);
            co2scrubber_numbers = co2scrubber_numbers.into_iter()
                .filter(|co2scrubber_number| co2scrubber_number.as_bytes()[i] as char == epsilon_rate[i])
                .collect::<Vec<&str>>();
        }
    }

    return (o2gen_numbers[0], co2scrubber_numbers[0]);
}


#[cfg(test)]
mod tests {
    use crate::day_3::*;

    const TEST_INPUT: &str = "00100 11110 10110 10111 10101 01111 00111 11100 10000 11001 00010 01010";

    #[test]
    fn test_get_gamma_and_epsilon_rates() {
        let numbers = TEST_INPUT.split_whitespace().collect::<Vec<&str>>();
        let (gamma_rate, epsilon_rate) = get_gamma_and_epsilon_rates(&numbers);
        assert_eq!(gamma_rate.iter().collect::<String>(), "10110");
        assert_eq!(epsilon_rate.iter().collect::<String>(), "01001");
    }

    #[test]
    fn test_get_o2gen_and_co2scrubber_rates() {
        let numbers = TEST_INPUT.split_whitespace().collect::<Vec<&str>>();
        let (o2gen_rate, co2scrubber_rate) = get_o2gen_and_co2scrubber_rates(&numbers);
        assert_eq!(o2gen_rate, "10111");
        assert_eq!(co2scrubber_rate, "01010");
    }
}
