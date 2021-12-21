use std::collections::HashSet;

pub fn part_1(lines: &Vec<String>) {
    if let Some(first_line) = lines.iter().filter(|line| !line.trim().is_empty()).next() {
        if let Some((min_x, max_x, min_y, max_y)) = parse_line(first_line) {
            println!("Highest: {:?}", solve(min_x, max_x, min_y, max_y));
        } else {
            panic!("Could not parse line: {}", first_line);
        }
    } else {
        panic!("No line with content provided!");
    }
}

pub fn part_2(lines: &Vec<String>) {
    println!("{:?}", lines);
}


fn parse_line(line: &str) -> Option<(i64, i64, i64, i64)> {
    let (_, bounds_info) = line.split_at(13);
    let whitespace_index = bounds_info.find(", ")?;
    let (x_info, y_info) = bounds_info.split_at(whitespace_index);
    let (_, x_bounds) = x_info.split_at(2);
    let (_, y_bounds) = y_info.split_at(4);
    let x_bounds_split = x_bounds.split("..");
    let y_bounds_split = y_bounds.split("..");
    let parsed: Vec<i64> = x_bounds_split.chain(y_bounds_split).filter_map(|bound| bound.parse::<i64>().ok()).collect();
    let min_x = parsed.get(0)?;
    let max_x = parsed.get(1)?;
    let min_y = parsed.get(2)?;
    let max_y = parsed.get(3)?;
    Some((*min_x, *max_x, *min_y, *max_y))
}

//fn calc_sx(vx_0: i64, t: i64) -> i64 {
//    if t < 0 {
//        panic!("t cannot be negative!");
//    }
//    if vx_0 <= t { (vx_0 + (-1 * (t - 1)) / 2) * t } else { vx_0 * (vx_0 + 1) / 2 }
//}
//
//fn calc_s(u: i64, t: i64, a: i64) -> i64 {
//    (u * 2 + a * t) * t / 2
//}
//
//fn calc_sy(vy_0: i64, t: i64) -> i64 {
//    if t < 0 {
//        panic!("t cannot be negative!");
//    }
//    (2 * vy_0 - t + 1) * t / 2
//}
//
//fn find_highest_point(vy_0: i64) -> (i64, i64) {
//    let mut sy = 0;
//    let mut t = 0;
//    for v in (0..=vy_0).rev() {
//        t += 1;
//        sy += v;
//    }
//    (sy, t)
//}
//
//fn solve(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Option<(i64, i64)> {
//    let mut highest_y_found = None;
//    let mut highest_x_found = None;
//
//    let mut vy_0 = 0;
//    loop {
//        let t_in_range_y = find_t_in_range(0, sy_highest, t_for_sy_highest, min_y, max_y, -1, false).into_iter().collect::<HashSet<i64>>();
//        println!("{:?}", t_in_range_y);
//        if t_in_range_y.is_empty() {
//            break;
//        }
//
//        let mut vx_0 = 0;
//        loop {
//            let t_in_range_x = find_t_in_range(vx_0, 0, 0, min_x, max_x, -1, true).into_iter().collect::<HashSet<i64>>();
//            if t_in_range_x.is_empty() {
//                break;
//            }
//
//            let overlapping = t_in_range_x.intersection(&t_in_range_y).collect::<Vec<&i64>>();
//            if let Some(t) = overlapping.first() {
//                if vy_0 > highest_y_found.unwrap_or(0) {
//                    highest_y_found = Some(vy_0);
//                    highest_x_found = Some(vx_0);
//                    println!("Found working: {} {} {}", vx_0, vy_0, t);
//                    break;
//                }
//            }
//            if highest_x_found.is_some() || vx_0 >= i64::MAX / 2 {
//                break;
//            }
//            vx_0 += 1;
//        }
//
//        if vy_0 == i64::MAX {
//            break;
//        }
//        vy_0 += 1;
//    }
//
//    highest_x_found.zip(highest_y_found)
//}
//
//fn find_t_in_range(v_0: i64, s_0: i64, t_0: i64, min: i64, max: i64, is_x: bool) -> Vec<i64> {
//    let distance_min = min - s_0;
//    let distance_max = max - s_0;
//    let a = -1;
//    // t^2 - 2 v_0 t + s = 0
//    // t =  v_0 +- (sqrt(4 (v_0^2 - s)) / 2)
//    let min_exists = v_0 * v_0 > distance_min;
//    let max_exists = v_0 * v_0 > distance_max;
//    if is_x && min_exists && max_exists {
//        let min_t = v_0 as f64 - ((4 * (v_0 * v_0 - distance_min)) as f64).sqrt() / 2.0;
//        let max_t = v_0 as f64 - ((4 * (v_0 * v_0 - distance_max)) as f64).sqrt() / 2.0;
//        ((min_t.round() as usize)..(max_t.round() as usize)).map(|i| i as i64).collect()
//    } else if !is_x && min_exists && max_exists {
//        let min_t = v_0 as f64 + ((4 * (v_0 * v_0 - distance_min)) as f64).sqrt() / 2.0;
//        let max_t = v_0 as f64 + ((4 * (v_0 * v_0 - distance_max)) as f64).sqrt() / 2.0;
//        ((min_t.round() as usize)..(max_t.round() as usize)).map(|i| i as i64).collect()
//    } else {
//        vec![]
//    }
//}

fn solve(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Option<(i64, i64)> {
    let mut highest_vy_0 = -1;
    let mut found = None;
    let mut vy_0 = 0;
    loop {
        if let Some(vx_0) = vx0_if_vy0_valid(vy_0, min_x, max_x, min_y, max_y) {
            if vy_0 > highest_vy_0 {
                println!("Found {} {}", vx_0, vy_0);
                highest_vy_0 = vy_0;
                found = Some((vx_0, vy_0));
            }
        } else {
            println!("No vx0 for vy0 of {}", vy_0);
            break;
        }
        vy_0 += 1;
    }
    found
}

fn vx0_if_vy0_valid(vy_0: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Option<i64> {
    let (min_t_y, max_t_y) = check_vy0(vy_0, min_y, max_y)?;
    let mut vx_0 = 0;
    // There's no point checking beyond when vx_0 is larger than max_x because that means that x
    // will go immediately to a point beyond our range.
    while vx_0 < max_x {
        if let Some((min_t_x, max_t_x)) = check_vx0(vx_0, min_x, max_x) {
            println!("Checking {} {} {} {}", min_t_y, max_t_y, min_t_x, max_t_x);
            if (min_t_y..=max_t_y).collect::<HashSet<i64>>().intersection(&(min_t_x..=max_t_x).collect::<HashSet<i64>>()).count() > 0 {
                return Some(vx_0);
            }
        }
        vx_0 += 1;
    }
    None
}

/// Assuming vy_0 > 0, min_y and max_y are both < 0, and min_y <= max_y
fn check_vy0(vy_0: i64, min_y: i64, max_y: i64) -> Option<(i64, i64)> {
    let f64_max = f64::MAX as i64;
    if vy_0 > f64_max || min_y > f64_max || max_y > f64_max ||
       vy_0 < 0 || min_y > 0 || max_y > 0 || min_y > max_y {
       panic!("Parameters do not fit assumptions! {} {} {}", vy_0, min_y, max_y);
    }

    // Because vy_0 > 0 and acceleration is -1, y goes up, then back down, and because both min_y
    // and max_y are negative, y will hit max_y first, then min_y.
    let min_t_y_pair = find_t_suvat(vy_0 as f64, max_y as f64, -1.0)?;
    let max_t_y_pair = find_t_suvat(vy_0 as f64, min_y as f64, -1.0)?;
    let min_t_y = if min_t_y_pair.0 >= 0.0 { min_t_y_pair.0 } else { min_t_y_pair.1 };
    let max_t_y = if max_t_y_pair.0 >= 0.0 { max_t_y_pair.0 } else { max_t_y_pair.1 };

    // Because we only check at integer values for t, the closest integer value to min_t and max_t
    // will be the ones actually checked for. This also means that if upon rounding, all integer values
    // from min_t_rounded .. max_t_rounded lie outside the range of min_t <= t <= max_t, we can
    // return None as this means vy_0 is too high.
    let min_t_y_rounded = min_t_y.round();
    let max_t_y_rounded = max_t_y.round();
    println!("Required {} {} {} {}", min_t_y, max_t_y, min_t_y_rounded, max_t_y_rounded);
    if ((min_t_y_rounded as usize)..=(max_t_y_rounded as usize)).any(|t| min_t_y <= t as f64 && t as f64 <= max_t_y) {
        Some((min_t_y_rounded as i64, max_t_y_rounded as i64))
    } else {
        None
    }
}

/// Assuming vx_0 > 0, min_x and max_x are both > 0, and min_x <= max_x
fn check_vx0(vx_0: i64, min_x: i64, max_x: i64) -> Option<(i64, i64)> {
    let f64_max = f64::MAX as i64;
    if vx_0 > f64_max || min_x > f64_max || max_x > f64_max ||
       vx_0 < 0 || min_x < 0 || max_x < 0 || min_x > max_x {
       panic!("Parameters do not fit assumptions! {} {} {}", vx_0, min_x, max_x);
    }

    // In the case that vx_0 is actually enough to reach both min_x and max_x, the lowest t in both
    // cases is taken because the quadratic formula will include the case that x continues
    // decelerating and starts going backwards, which will not actually happen in our model.
    let min_t_x_pair = find_t_suvat(vx_0 as f64, min_x as f64, -1.0)?;
    let max_t_x_pair = find_t_suvat(vx_0 as f64, max_x as f64, -1.0)?;
    let min_t_x = if min_t_x_pair.0 < min_t_x_pair.1 { min_t_x_pair.0 } else { min_t_x_pair.1 };
    let max_t_x = if max_t_x_pair.0 < max_t_x_pair.1 { max_t_x_pair.0 } else { max_t_x_pair.1 };

    let min_t_x_rounded = min_t_x.round();
    let max_t_x_rounded = max_t_x.round();
    if ((min_t_x_rounded as usize)..=(max_t_x_rounded as usize)).any(|t| min_t_x <= t as f64 && t as f64 <= max_t_x) {
        Some((min_t_x_rounded as i64, max_t_x_rounded as i64))
    } else {
        None
    }
}

/// Rearranging the suvat equation in t, u, s, and a results in a quadratic.
fn find_t_suvat(u: f64, s: f64, a: f64) -> Option<(f64, f64)> {
    let determinant = u * u + 2.0 * a * s;
    if determinant < 0.0 {
        None
    } else {
        Some(((-u + determinant.sqrt()) / a, (-u - determinant.sqrt()) / a))
    }
}

//fn find_t_triangular(u: f64, s: f64, is_x: bool) -> Option<(f64, f64)> {
//    None
//}


#[cfg(test)]
mod tests {
    //use crate::day_17::*;

    const TEST_INPUT_1: &str = r#"
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        get_test_input(TEST_INPUT_1);
    }

    #[test]
    fn test_part_2() {
    }
}
