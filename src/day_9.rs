use ndarray::{Array2, Axis};
use std::collections::HashSet;


/// Parse the heightmap from the given input lines
fn parse_heightmap(lines: &Vec<String>) -> Array2<u8> {
    if lines.is_empty() {
        return Array2::zeros((0, 0));
    }
    let width = lines.len();
    let height = lines[0].len();
    let mut heightmap = Array2::<u8>::zeros((width, height));
    for (i, mut row) in heightmap.axis_iter_mut(Axis(0)).enumerate() {
        let line_as_vec: Vec<u8> = lines[i].chars().into_iter().filter_map(|c| c.to_string().parse::<u8>().ok()).collect();
        for j in 0..row.len() {
            row[j] = line_as_vec[j];
        }
    }
    heightmap
}

/// Get all points around (i, j) in the heightmap (points at the boundary will have less
/// surrounding points)
fn surrounding_points(heightmap: &Array2<u8>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let shape = heightmap.shape();
    let below = if i < shape[0] - 1 { Some((i + 1, j)) } else { None };
    let right = if j < shape[1] - 1 { Some((i, j + 1)) } else { None };
    let up    = if i > 0            { Some((i - 1, j)) } else { None };
    let left  = if j > 0            { Some((i, j - 1)) } else { None };
    vec![below, right, up, left].into_iter().filter_map(|p| p).collect()
}

/// Find all lowest points by iterating through all points and checking all points around them are
/// higher.
fn find_all_low_points(heightmap: &Array2<u8>) -> Vec<(usize, usize)> {
    let shape = heightmap.shape();
    let mut low_point_indices: Vec<(usize, usize)> = vec![];
    for i in 0..shape[0] {
        for j in 0..shape[1] {
            let current = heightmap[[i, j]];
            let surrounding = surrounding_points(heightmap, i, j);
            if surrounding.iter().all(|(i_other, j_other)| current < heightmap[[*i_other, *j_other]]) {
                low_point_indices.push((i, j));
            }
        }
    }
    low_point_indices
}


fn get_low_point_risk_level_sum(heightmap: &Array2<u8>) -> i64 {
    let low_points = find_all_low_points(heightmap);
    low_points.into_iter().fold(0, |acc, (i, j)| acc + heightmap[[i, j]] as i64 + 1)
}

/// For part 1, we simply find all lowest points using the utility methods, then sum them.
pub fn part_1(lines: &Vec<String>) {
    let lines_filtered: Vec<String> = lines.iter().filter(|line| !line.is_empty()).map(|line| line.clone()).collect();
    let heightmap = parse_heightmap(&lines_filtered);
    let sum = get_low_point_risk_level_sum(&heightmap);
    println!("Sum of low point risk levels: {}", sum);
}


fn get_basin_sizes_multiplied(heightmap: &Array2<u8>, low_point_indices: Vec<(usize, usize)>) -> i64 {
    let mut sizes = vec![];
    for (low_point_i, low_point_j) in low_point_indices {
        let mut boundary: Vec<(usize, usize)> = vec![(low_point_i, low_point_j)];
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        while !boundary.is_empty() {
            let (i, j) = boundary.remove(0);
            seen.insert((i, j));
            let mut new_boundary_points: Vec<(usize, usize)> = surrounding_points(heightmap, i, j)
                .into_iter()
                .filter(|(k, l)| !seen.contains(&(*k, *l)) && !boundary.contains(&(*k, *l)) &&
                    heightmap[[*k, *l]] != 9 && heightmap[[i, j]] < heightmap[[*k, *l]])
                .collect();
            boundary.append(&mut new_boundary_points);
        }
        sizes.push(seen.len() as i64);
    }
    sizes.sort();
    sizes.into_iter().rev().take(3).fold(1, |acc, size| acc * size)
}

/// For part 2, we first find the lowest points using the existing method, then do a breadth-first
/// search of the heightmap starting from each lowest point, and ending when we reach a 9
/// (we also check the points we are adding while expanding the boundary are higher than existing
/// points but this isn't actually necessary, we only really needed to check for 9s).
/// Then once we have found the sizes of all basins we multiply the three largest sizes together.
pub fn part_2(lines: &Vec<String>) {
    let lines_filtered: Vec<String> = lines.iter().filter(|line| !line.is_empty()).map(|line| line.clone()).collect();
    let heightmap = parse_heightmap(&lines_filtered);
    let low_points = find_all_low_points(&heightmap);
    let basin_sizes = get_basin_sizes_multiplied(&heightmap, low_points);
    println!("Basin Sizes Multiplied: {}", basin_sizes);
}


#[cfg(test)]
mod tests {
    use crate::day_9::*;

    const TEST_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_low_point_risk_sum() {
        let test_input: Vec<String> = TEST_INPUT.split_whitespace().map(str::to_string).collect();
        assert_eq!(get_low_point_risk_level_sum(&parse_heightmap(&test_input)), 15);
    }

    #[test]
    fn test_2() {
        let test_input: Vec<String> = TEST_INPUT.split_whitespace().map(str::to_string).collect();
        let lines_filtered: Vec<String> = test_input.iter().filter(|line| !line.is_empty()).map(|line| line.clone()).collect();
        let heightmap = parse_heightmap(&lines_filtered);
        let low_points = find_all_low_points(&heightmap);
        let basin_sizes = get_basin_sizes_multiplied(&heightmap, low_points);
        assert_eq!(basin_sizes, 1134);
    }
}
