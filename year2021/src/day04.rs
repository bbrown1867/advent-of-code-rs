use ndarray::Array2;
use regex::Regex;
use std::fs;

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = 5;

pub fn main() {
    let (numbers_drawn, bingo_boards) = parse_bingo_game("data/day04.txt");

    let board_idx = find_first_winning_board(&numbers_drawn, &bingo_boards);
    println!(
        "day04 part1: {}",
        get_special_value_for_winning_board(
            &numbers_drawn,
            &bingo_boards[board_idx]
        )
    );

    let board_idx = find_last_winning_board(&numbers_drawn, &bingo_boards);
    println!(
        "day04 part2: {}",
        get_special_value_for_winning_board(
            &numbers_drawn,
            &bingo_boards[board_idx]
        )
    );
}

fn parse_bingo_game(path: &str) -> (Vec<i32>, Vec<Array2<i32>>) {
    let contents = fs::read_to_string(path).expect("Could not read file.");

    let mut lines = contents.split("\n");

    let numbers_drawn: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    // Skip the first empty line
    lines.next().unwrap();

    let mut bingo_boards = Vec::new();

    let mut curr_row = 0;
    let mut curr_board = Array2::zeros((NUM_ROWS, NUM_COLS));
    for line in lines {
        if line.len() == 0 {
            bingo_boards.push(curr_board);
            curr_row = 0;
            curr_board = Array2::zeros((NUM_ROWS, NUM_COLS));
        } else {
            let mut curr_col = 0;
            let re = Regex::new(r"(\d+)").unwrap();
            for cap in re.captures_iter(line) {
                let val: &str = &cap[1];
                curr_board[[curr_row, curr_col]] = val.parse::<i32>().unwrap();
                curr_col += 1;
            }

            curr_row += 1;
        }
    }

    (numbers_drawn, bingo_boards)
}

fn is_marker_board_bingo(marker_board: &Array2<i32>) -> bool {
    for row in marker_board.rows() {
        if row.sum() == NUM_ROWS as i32 {
            return true;
        }
    }

    for col in marker_board.columns() {
        if col.sum() == NUM_COLS as i32 {
            return true;
        }
    }

    false
}

fn find_first_winning_board(
    numbers_drawn: &Vec<i32>,
    bingo_boards: &Vec<Array2<i32>>,
) -> usize {
    let mut is_drawn: Vec<Array2<i32>> = Vec::new();
    for _ in bingo_boards {
        is_drawn.push(Array2::zeros((NUM_ROWS, NUM_COLS)));
    }

    for num in numbers_drawn {
        for (idx, board) in bingo_boards.iter().enumerate() {
            for i in 0..NUM_ROWS {
                for j in 0..NUM_COLS {
                    if board[[i, j]] == *num {
                        is_drawn[idx][[i, j]] = 1;
                    }
                }
            }

            if is_marker_board_bingo(&is_drawn[idx]) {
                return idx;
            }
        }
    }

    panic!("No winning board found.");
}

fn find_last_winning_board(
    numbers_drawn: &Vec<i32>,
    bingo_boards: &Vec<Array2<i32>>,
) -> usize {
    let mut winning_boards: Vec<usize> = Vec::new();

    let mut is_drawn: Vec<Array2<i32>> = Vec::new();
    for _ in bingo_boards {
        is_drawn.push(Array2::zeros((NUM_ROWS, NUM_COLS)));
    }

    for num in numbers_drawn {
        for (idx, board) in bingo_boards.iter().enumerate() {
            for i in 0..NUM_ROWS {
                for j in 0..NUM_COLS {
                    if board[[i, j]] == *num {
                        is_drawn[idx][[i, j]] = 1;
                    }
                }
            }

            if is_marker_board_bingo(&is_drawn[idx]) {
                if !winning_boards.contains(&idx) {
                    winning_boards.push(idx);
                }
            }
        }
    }

    winning_boards.pop().expect("No winning board found.")
}

fn get_special_value_for_winning_board(
    numbers_drawn: &Vec<i32>,
    winning_board: &Array2<i32>,
) -> i32 {
    let mut marker_board = Array2::zeros((NUM_ROWS, NUM_COLS));
    let mut cleared_board = winning_board.clone();

    for num in numbers_drawn {
        for i in 0..NUM_ROWS {
            for j in 0..NUM_COLS {
                if winning_board[[i, j]] == *num {
                    marker_board[[i, j]] = 1;
                    cleared_board[[i, j]] = 0;
                }
            }
        }

        if is_marker_board_bingo(&marker_board) {
            return *num * cleared_board.sum();
        }
    }

    panic!("Board did not win.");
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::arr2;

    fn get_test_input() -> (Vec<i32>, Vec<Array2<i32>>) {
        let numbers_drawn = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25,
            12, 22, 18, 20, 8, 19, 3, 26, 1,
        ];

        let b1 = arr2(&[
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]);

        let b2 = arr2(&[
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]);

        let b3 = arr2(&[
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]);

        let bingo_boards = vec![b1, b2, b3];

        (numbers_drawn, bingo_boards)
    }

    #[test]
    fn test_find_first_winning_board() {
        let (numbers_drawn, bingo_boards) = get_test_input();
        assert_eq!(find_first_winning_board(&numbers_drawn, &bingo_boards), 2);
    }

    #[test]
    fn test_find_last_winning_board() {
        let (numbers_drawn, bingo_boards) = get_test_input();
        assert_eq!(find_last_winning_board(&numbers_drawn, &bingo_boards), 1);
    }

    #[test]
    fn test_get_special_value_for_winning_board() {
        let (numbers_drawn, bingo_boards) = get_test_input();
        let winning_board = &bingo_boards[2];
        assert_eq!(
            get_special_value_for_winning_board(&numbers_drawn, winning_board),
            4512
        );
    }
}
