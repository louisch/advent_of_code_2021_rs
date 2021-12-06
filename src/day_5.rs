use std::collections::HashMap;
use std::convert::TryInto;


#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x: x, y: y }
    }

    fn from_parts(parts: &Vec<i64>) -> Self {
        Point { x: parts[0], y: parts[1] }
    }
}

#[derive(Debug)]
struct VentLine {
    p1: Point,
    p2: Point,
}

impl VentLine {
    fn from_string(s: &String) -> anyhow::Result<Self> {
        let parts = s.split(" -> ")
            .map(|part| part.split(',').filter_map(|coord| coord.parse::<i64>().ok()).collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("{} needs to be two parts separated by a ' -> '", s));
        }
        let part1 = &parts[0];
        let part2 = &parts[1];
        if part1.len() != 2 {
            return Err(anyhow::anyhow!("First part of a vent line {:?} needs to be two numbers separated by a comma", part1));
        }
        if part2.len() != 2 {
            return Err(anyhow::anyhow!("Second part of a vent line {:?} needs to be two numbers separated by a comma", part2));
        }
        Ok(VentLine { p1: Point::from_parts(part1), p2: Point::from_parts(part2) })
    }

    fn is_orthogonal(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn points_in_orthogonal_line(&self) -> Vec<Point> {
        if self.p1.x == self.p2.x {
            let (y1, y2) = if self.p1.y < self.p2.y { (self.p1.y, self.p2.y) } else { (self.p2.y, self.p1.y) };
            (y1..y2 + 1).map(|y| Point::new(self.p1.x, y)).collect::<Vec<Point>>()
        } else if self.p1.y == self.p2.y {
            let (x1, x2) = if self.p1.x < self.p2.x { (self.p1.x, self.p2.x) } else { (self.p2.x, self.p1.x) };
            (x1..x2 + 1).map(|x| Point::new(x, self.p1.y)).collect::<Vec<Point>>()
        } else {
            vec![]
        }
    }

    fn points_in_line(&self) -> Vec<Point> {
        if self.p1.x == self.p2.x {
            let (y1, y2) = if self.p1.y < self.p2.y { (self.p1.y, self.p2.y) } else { (self.p2.y, self.p1.y) };
            (y1..y2 + 1).map(|y| Point::new(self.p1.x, y)).collect::<Vec<Point>>()
        } else if self.p1.y == self.p2.y {
            let (x1, x2) = if self.p1.x < self.p2.x { (self.p1.x, self.p2.x) } else { (self.p2.x, self.p1.x) };
            (x1..x2 + 1).map(|x| Point::new(x, self.p1.y)).collect::<Vec<Point>>()
        } else {
            let xs = if self.p1.x < self.p2.x {
                (self.p1.x..=self.p2.x).collect::<Vec<i64>>()
            } else {
                (self.p2.x..=self.p1.x).rev().collect::<Vec<i64>>()
            };
            let ys = if self.p1.y < self.p2.y {
                (self.p1.y..=self.p2.y).collect::<Vec<i64>>()
            } else {
                (self.p2.y..=self.p1.y).rev().collect::<Vec<i64>>()
            };
            xs.iter().zip(ys).map(|(x, y)| Point::new(*x, y)).collect::<Vec<Point>>()
        }
    }
}

fn parse_ventlines(input: &Vec<String>) -> Vec<VentLine> {
    input.into_iter()
        .filter(|input_line| !input_line.trim().is_empty())
        .map(|input_line| VentLine::from_string(&input_line.trim().to_string()).unwrap())
        .collect::<Vec<VentLine>>()
}

fn parse_orthogonal_ventlines(input: &Vec<String>) -> Vec<VentLine> {
    parse_ventlines(input).into_iter()
        .filter(|line| line.is_orthogonal())
        .collect::<Vec<VentLine>>()
}

pub fn count_overlapping_orthogonal_ventlines(input: &Vec<String>) -> i64 {
    let ventlines = parse_orthogonal_ventlines(input);
    let mut map: HashMap<Point, i64> = HashMap::new();
    for ventline in ventlines {
        for point in ventline.points_in_orthogonal_line() {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    map.values().filter(|count| **count >= 2).count().try_into().unwrap()
}

pub fn count_overlapping_ventlines(input: &Vec<String>) -> i64 {
    let ventlines = parse_ventlines(input);
    let mut map: HashMap<Point, i64> = HashMap::new();
    for ventline in ventlines {
        for point in ventline.points_in_line() {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    map.values().filter(|count| **count >= 2).count().try_into().unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day_5::*;

    const TEST_INPUT: &str = r#"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    "#;

    #[test]
    fn test_count_orthogonal() {
        let test_input = TEST_INPUT.trim().split('\n').map(|line| line.trim().to_string()).collect::<Vec<String>>();
        assert_eq!(count_overlapping_orthogonal_ventlines(&test_input), 5);
    }

    #[test]
    fn test_count_all() {
        let test_input = TEST_INPUT.trim().split('\n').map(|line| line.trim().to_string()).collect::<Vec<String>>();
        assert_eq!(count_overlapping_ventlines(&test_input), 12);
    }
}
