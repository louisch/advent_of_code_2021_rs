use ndarray::{Array2, Array3};


pub fn part_1(lines: &Vec<String>) {
}

pub fn part_2(lines: &Vec<String>) {
}


struct ScannerList {
    index: usize,
    beacons: Array2<i64>,
}

fn parse_scanner_list(lines: &Vec<String>) -> ScannerList {
    let result = ScannerList { index: 0, beacons: Array2::zeros((0, 0)) };
    result
}

fn create_distance_matrix(scanner_list: &ScannerList) -> Array2<i64> {
    let scanner_list_clone = scanner_list.clone();
    let len_beacons = scanner_list.beacons.shape()[0];
    let mut result = Array2::zeros((len_beacons, len_beacons));
    for (i, beacon1) in scanner_list.beacons.outer_iter().enumerate() {
        for (j, beacon2) in scanner_list.beacons.outer_iter().enumerate() {
            //let distance = beacon2 - beacon1;
        }
    }
    result
}



#[cfg(test)]
mod tests {
    use crate::day_19::*;

    const TEST_INPUT_EXAMPLE: &str = r#"
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test() {
    }
}
