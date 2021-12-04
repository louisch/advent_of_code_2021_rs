use ndarray::{Array2, ArrayView};


#[derive(Debug)]
pub struct BingoSquare {
    pub number: i64,
    pub marked: bool,
}

impl num_traits::identities::Zero for BingoSquare {
    fn zero() -> Self {
        BingoSquare::new(0, false)
    }

    fn is_zero(&self) -> bool {
        self.number == 0
    }
}

impl std::ops::Add for BingoSquare {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BingoSquare::new(self.number + other.number, false)
    }
}

impl std::clone::Clone for BingoSquare {
    fn clone(&self) -> Self {
        BingoSquare::new(self.number, self.marked)
    }
}

impl BingoSquare {
    fn new(number: i64, marked: bool) -> Self {
        Self {
            number: number,
            marked: marked,
        }
    }
}

impl PartialEq<BingoSquare> for BingoSquare {
    fn eq(&self, other: &BingoSquare) -> bool {
        return self.number == other.number;
    }
}


fn parse_bingo_numbers(line: &str) -> Vec<i64> {
    line.split(',').filter_map(|number| number.parse::<i64>().ok()).collect::<Vec<i64>>()
}

fn parse_bingo_board(lines: &Vec<&str>) -> Array2<BingoSquare> {
    let mut matrix = Array2::<BingoSquare>::zeros((0, 5));
    for line in lines {
        let row = line.split_whitespace()
                      .map(|number| BingoSquare { number: number.parse::<i64>().unwrap(), marked: false })
                      .collect::<Vec<BingoSquare>>();
        if let Err(e) = matrix.push_row(ArrayView::from(&row)) {
            panic!("Could not push row {}, onto matrix, error {}", line, e);
        }
    }
    return matrix;
}

fn parse_bingo_boards(lines: &Vec<&str>) -> Vec<Array2<BingoSquare>> {
    let mut matrices = vec![];
    let mut matrix_lines = vec![];
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            if !matrix_lines.is_empty() {
                matrices.push(parse_bingo_board(&matrix_lines));
            }
            matrix_lines = vec![];
            continue;
        }
        matrix_lines.push(trimmed_line);
    }
    return matrices;
}

fn is_winning(board: &Array2<BingoSquare>) -> bool {
    for row in board.rows() {
        if row.iter().all(|square| square.marked) {
            return true;
        }
    }
    for column in board.columns() {
        if column.iter().all(|square| square.marked) {
            return true;
        }
    }
    false
}

pub fn find_winning_bingo_board(lines: &mut Vec<&str>) -> Option<(i64, Array2<BingoSquare>)> {
    if lines.is_empty() {
        return None;
    }

    let first_line = lines.remove(0);
    let bingo_numbers = parse_bingo_numbers(first_line);
    let mut bingo_boards = parse_bingo_boards(lines);

    let mut winning_number_and_board = None;
    for bingo_number in bingo_numbers {
        for b_index in 0..bingo_boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if bingo_boards[b_index][[i, j]].number == bingo_number {
                        bingo_boards[b_index][[i, j]].marked = true;
                    }
                }
            }
            if is_winning(&bingo_boards[b_index]) {
                winning_number_and_board = Some((bingo_number, b_index));
                break;
            }
        }
        if winning_number_and_board.is_some() {
            break;
        }
    }
    winning_number_and_board.map(|(winning_number, b_index)| (winning_number, bingo_boards[b_index].clone()))
}

pub fn find_last_to_win_bingo_board(lines: &mut Vec<&str>) -> Option<(i64, Array2<BingoSquare>)> {
    if lines.is_empty() {
        return None;
    }

    let first_line = lines.remove(0);
    let bingo_numbers = parse_bingo_numbers(first_line);
    let mut bingo_boards = parse_bingo_boards(lines);

    let mut last_number = 0;
    for bingo_number in bingo_numbers {
        let mut winning_board_indices = vec![];
        for b_index in 0..bingo_boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if bingo_boards[b_index][[i, j]].number == bingo_number {
                        bingo_boards[b_index][[i, j]].marked = true;
                    }
                }
            }
            if is_winning(&bingo_boards[b_index]) {
                winning_board_indices.push(b_index);
            }
        }

        if winning_board_indices.len() == bingo_boards.len() {
            last_number = bingo_number;
            break;
        }
        bingo_boards.retain(|board| !is_winning(&board));
    }

    Some((last_number, bingo_boards[0].clone()))
}

pub fn get_score(number: i64, board: &Array2<BingoSquare>) -> i64 {
    let unmarked_numbers = board.iter()
        .filter(|square| !square.marked)
        .map(|square| square.number)
        .collect::<Vec<i64>>();
    number * unmarked_numbers.iter().fold(0, |x, y| x + y)
}


#[cfg(test)]
mod tests {
    use crate::day_4::*;

    const TEST_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
    "#;

    #[test]
    fn test_find_winning_bingo_board() {
        let winner = find_winning_bingo_board(&mut TEST_INPUT.split("\n").collect::<Vec<&str>>());
        let expected_board = ndarray::array![[14, 21, 17, 24, 4], [10, 16, 15, 9, 19], [18, 8, 23, 26, 20], [22, 11, 13, 6, 5], [2, 0, 12, 3, 7]].map(|number| BingoSquare::new(*number, false));
        assert!(winner.is_some());
        if let Some((winning_number, winning_board)) = winner {
            assert_eq!(get_score(winning_number, &winning_board), 4512);
            assert_eq!(winning_number, 24);
            assert_eq!(winning_board, expected_board);
        }
    }

    #[test]
    fn test_find_last_winning_bingo_board() {
        let last = find_last_to_win_bingo_board(&mut TEST_INPUT.split("\n").collect::<Vec<&str>>());
        let expected_board = ndarray::array![[3, 15, 0, 2, 22], [9, 18, 13, 17, 5], [19, 8, 7, 25, 23], [20, 11, 10, 24, 4], [14, 21, 16, 12, 6]].map(|number| BingoSquare::new(*number, false));
        assert!(last.is_some());
        if let Some((number, board)) = last {
            assert_eq!(board, expected_board);
            assert_eq!(get_score(number, &board), 1924);
            assert_eq!(number, 13);
        }
    }
}
