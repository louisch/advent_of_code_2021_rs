const NEW_FISH_TIMER: usize = 8;
const RESET_FISH_TIMER: usize = 6;

fn parse_initial_state(input: &Vec<String>) -> Vec<i64> {
    let mut counts = vec![0; NEW_FISH_TIMER + 1];
    for timer in input[0].split(',') {
        let timer_parsed = timer.parse::<i64>();
        if let Ok(timer_as_number) = timer_parsed {
            let timer_as_usize = timer_as_number as usize;
            counts[timer_as_usize] += 1;
        }
    }
    counts
}

fn advance_timer_once(fish_state: &mut Vec<i64>) {
    let fish_giving_birth = fish_state[0];
    for i in 0..NEW_FISH_TIMER {
        fish_state[i] = fish_state[i + 1];
    }
    fish_state[NEW_FISH_TIMER] = fish_giving_birth;
    fish_state[RESET_FISH_TIMER] += fish_giving_birth;
}

fn advance_timer(fish_state: &mut Vec<i64>, days: i64) {
    for _ in 0..days {
        advance_timer_once(fish_state);
    }
}

pub fn count_lanternfish(input: &Vec<String>, days: i64) -> i64 {
    let mut fish_state = parse_initial_state(input);
    advance_timer(&mut fish_state, days);
    fish_state.iter().fold(0, |x, y| x + y)
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
