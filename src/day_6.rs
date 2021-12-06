use std::convert::TryInto;


#[derive(Debug)]
struct Lanternfish {
    timer: i64,
}

impl Lanternfish {
    const NEW_FISH_TIMER: i64 = 8;
    const RESET_FISH_TIMER: i64 = 6;

    fn new(timer: i64) -> Self {
        Lanternfish { timer: timer }
    }

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let timer = s.parse::<i64>()?;
        Ok(Lanternfish::new(timer))
    }
}

fn parse_initial_state(input: &Vec<String>) -> Vec<Lanternfish> {
    input[0].split(',').filter_map(|s| Lanternfish::from_str(s).ok()).collect::<Vec<Lanternfish>>()
}

fn advance_timer_once(all_fish: &mut Vec<Lanternfish>) {
    let mut new_fish = vec![];
    for fish in all_fish.iter_mut() {
        if fish.timer == 0 {
            new_fish.push(Lanternfish::new(Lanternfish::NEW_FISH_TIMER));
            fish.timer = Lanternfish::RESET_FISH_TIMER;
        } else {
            fish.timer -= 1;
        }
    }
    all_fish.append(&mut new_fish);
}

fn advance_timer(all_fish: &mut Vec<Lanternfish>, days: i64) {
    for _ in 0..days {
        advance_timer_once(all_fish);
    }
}

fn simulate_lanternfish(input: &Vec<String>, days: i64) -> Vec<Lanternfish> {
    let mut all_fish = parse_initial_state(input);
    advance_timer(&mut all_fish, days);
    all_fish
}

pub fn count_lanternfish(input: &Vec<String>, days: i64) -> i64 {
    simulate_lanternfish(input, days).len().try_into().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day_6::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_simulate_lanternfish() {
        let test_input = vec![TEST_INPUT.to_string()];
        assert_eq!(count_lanternfish(&test_input, 18), 26);
        assert_eq!(count_lanternfish(&test_input, 80), 5934);
        assert_eq!(count_lanternfish(&test_input, 256), 26984457539);
    }
}
