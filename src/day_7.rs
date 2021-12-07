fn parse_crabs(input: &Vec<String>) -> Vec<i64> {
    input[0].split(',').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>()
}

/// This does not necessarily find the actual median, but just the crab which has index == crabs.len() / 2
/// For odd lengths this is the actual median, but for even lengths this is the smaller of the two
/// crabs which is in the middle. This is still sufficient for an optimal position, however.
fn find_median(crabs: &mut Vec<i64>) -> i64 {
    crabs.sort();
    crabs[crabs.len() / 2]
}

fn calculate_fuel(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().fold(0, |acc, crab| acc + (crab - position).abs())
}

pub fn find_optimum_constant(input: &Vec<String>) -> (i64, i64) {
    let mut crabs = parse_crabs(input);
    let optimum = find_median(&mut crabs);
    (optimum, calculate_fuel(&crabs, optimum))
}

fn calculate_fuel_triangular(crabs: &Vec<i64>, position: i64) -> i64 {
    crabs.iter().fold(0, |acc, crab| {
        let diff = (crab - position).abs();
        acc + diff * (diff + 1) / 2
    })
}

/// For the second part of the problem, we use a search.
/// EDIT: it seems the arithmetic mean does work for most datasets here, because the triangular
/// number creates an n^2 term. However, in some rare cases there is a small error that can
/// potentially cause rounding errors.
pub fn find_optimum_triangular(input: &Vec<String>) -> (i64, i64) {
    let mut crabs = parse_crabs(input);
    crabs.sort();
    let mut position = crabs[crabs.len() / 2];
    let mut cost = calculate_fuel_triangular(&crabs, position);
    let mut cost_below = calculate_fuel_triangular(&crabs, position - 1); // We're not going to check boundary conditions because for this puzzle it doesn't seem necessary.
    let mut cost_above = calculate_fuel_triangular(&crabs, position + 1);
    while cost_below < cost || cost_above < cost {
        if cost_below < cost {
            position -= 1;
            cost_above = cost;
            cost = cost_below;
            cost_below = calculate_fuel_triangular(&crabs, position - 1);
        } else {
            position += 1;
            cost_below = cost;
            cost = cost_above;
            cost_above = calculate_fuel_triangular(&crabs, position + 1);
        }
    }
    (position, cost)
}


#[cfg(test)]
mod tests {
    use crate::day_7::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_find_optimum_constant() {
        let test_input = vec![TEST_INPUT.to_string()];
        assert_eq!(find_optimum_constant(&test_input), (2, 37));
    }

    #[test]
    fn test_find_optimum_triangular() {
        let test_input = vec![TEST_INPUT.to_string()];
        assert_eq!(find_optimum_triangular(&test_input), (5, 168));
    }
}
