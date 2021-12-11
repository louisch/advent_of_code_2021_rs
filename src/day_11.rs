use ndarray::Array2;


fn parse_octopuses(lines: &Vec<String>) -> Array2<i8> {
    if lines.is_empty() {
        return Array2::<i8>::zeros((0, 0));
    }
    let lines_filtered: Vec<String> = lines.iter().filter(|line| !line.is_empty()).map(|line| line.to_string()).collect();
    let shape = [lines_filtered.len(), lines_filtered[0].len()];
    let mut octopuses = Array2::zeros(shape);
    for (i, line) in lines_filtered.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as i8;
            octopuses[[i, j]] = digit;
        }
    }
    octopuses
}

fn step_n(octopuses: &mut Array2<i8>, n: u32) -> u64 {
    let mut count = 0;
    for _ in 0..n {
        count += step_once(octopuses);
    }
    count
}

fn step_once(octopuses: &mut Array2<i8>) -> u64 {
    *octopuses += 1;

    let (shape_i, shape_j) = octopuses.dim();
    let mut flashes = 0;
    loop {
        let current_flashes = flashes;
        for i in 0..shape_i {
            for j in 0..shape_j {
                let current_octopus = octopuses[[i, j]].clone();
                if current_octopus > 9 {
                    for adjacent_coords in all_adjacent_to((i, j), (shape_i, shape_j)).iter() {
                        let adjacent_octopus = octopuses[*adjacent_coords];
                        if adjacent_octopus >= 0 && adjacent_octopus <= 9 {
                            octopuses[*adjacent_coords] += 1;
                        }
                    }
                    octopuses[[i, j]] = -1;
                    flashes += 1;
                }
            }
        }
        if current_flashes == flashes {
            break;
        }
    }

    for octopus in octopuses.iter_mut() {
        if *octopus < 0 {
            *octopus = 0;
        }
    }

    flashes
}

fn all_adjacent_to((i_usize, j_usize): (usize, usize), (shape_i, shape_j): (usize, usize)) -> Vec<[usize; 2]> {
    let i = i_usize as i64;
    let j = j_usize as i64;
    let adjacent_coords = ((i-1)..=(i+1))
        .flat_map(move |i_coord| ((j-1)..=(j+1)).map(move |j_coord| (i_coord, j_coord)))
        .filter(|(i_coord, j_coord)|
                    *i_coord >= 0 && (*i_coord as usize) < shape_i &&
                    *j_coord >= 0 && (*j_coord as usize) < shape_j)
        .map(|(i_coord, j_coord)| [i_coord as usize, j_coord as usize])
        .collect();
    adjacent_coords
}

fn count_flashes_after_100_steps(lines: &Vec<String>) -> u64 {
    let mut octopuses = parse_octopuses(lines);
    step_n(&mut octopuses, 100)
}

pub fn part_1(lines: &Vec<String>) {
    let count = count_flashes_after_100_steps(lines);
    println!("Total flashes: {}", count);
}


fn simulate_until_synced(lines: &Vec<String>) -> u64 {
    let mut step = 0;
    let mut octopuses = parse_octopuses(lines);
    while !octopuses.iter().all(|octopus| *octopus == 0) {
        step_once(&mut octopuses);
        step += 1;
    }
    step
}

pub fn part_2(lines: &Vec<String>) {
    let step = simulate_until_synced(lines);
    println!("Step when synced: {}", step);
}


#[cfg(test)]
mod tests {
    use crate::day_11::*;

    const TEST_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn test_100_steps() {
        assert_eq!(count_flashes_after_100_steps(&get_test_input()), 1656);
    }

    #[test]
    fn test_until_synced() {
        assert_eq!(simulate_until_synced(&get_test_input()), 195);
    }
}
