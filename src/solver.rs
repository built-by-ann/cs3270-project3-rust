/*
 * Name: Ann Mathew
 * Email: your_email@vanderbilt.edu
 * Project: CS 3270 Project 3 - Rust Sudoku Solver
 */

use std::fs;

/** A Sudoku board represented as a 9x9 grid. */
pub type Board = [[u8; 9]; 9];

/**
 * Reads a Sudoku puzzle from the given file path.
 *
 * The file should contain 81 integer values. Values should be in the
 * range 0 through 9, where 0 represents an empty cell.
 *
 * @param path The path to the puzzle text file.
 * @return A 9x9 Sudoku board loaded from the file.
 */
pub fn read_puzzle(path: &str) -> Board {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read puzzle file: {}", path));

    let numbers: Vec<u8> = contents
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .map(|ch| ch.to_digit(10).expect("Failed to convert digit.") as u8)
        .collect();

    if numbers.len() != 81 {
        panic!(
            "Puzzle file must contain exactly 81 digits, but found {}.",
            numbers.len()
        );
    }

    let mut board = [[0u8; 9]; 9];

    for row in 0..9 {
        for col in 0..9 {
            board[row][col] = numbers[row * 9 + col];
        }
    }

    board
}

/**
 * Prints the Sudoku board with borders.
 *
 * @param board The Sudoku board to print.
 */
pub fn print_board(board: &Board) {
    println!("+-------+-------+-------+");

    for (row_index, row) in board.iter().enumerate() {
        print!("| ");

        for (col_index, value) in row.iter().enumerate() {
            print!("{} ", value);

            if col_index % 3 == 2 {
                print!("| ");
            }
        }

        println!();

        if row_index % 3 == 2 {
            println!("+-------+-------+-------+");
        }
    }
}

/**
 * Finds the next empty cell in the Sudoku board.
 *
 * An empty cell is represented by the value 0.
 *
 * @param board The Sudoku board to search.
 * @return The row and column of the empty cell, or None if no empty cell exists.
 */
pub fn find_empty(board: &Board) -> Option<(usize, usize)> {
    for (row, rows) in board.iter().enumerate() {
        for (col, value) in rows.iter().enumerate() {
            if *value == 0 {
                return Some((row, col));
            }
        }
    }

    None
}

/**
 * Determines whether a value can be placed at the specified position.
 *
 * The value must not already appear in the same row, same column,
 * or same 3x3 subgrid.
 *
 * @param board The Sudoku board.
 * @param row The row index.
 * @param col The column index.
 * @param value The value to place.
 * @return True if the value can be legally placed, otherwise false.
 */
pub fn is_valid(board: &Board, row: usize, col: usize, value: u8) -> bool {
    if board[row].contains(&value) {
        return false;
    }

    if board.iter().any(|r| r[col] == value) {
        return false;
    }

    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    for r in board.iter().skip(start_row).take(3) {
        if r.iter().skip(start_col).take(3).any(|&v| v == value) {
            return false;
        }
    }

    true
}

/**
 * Solves the Sudoku puzzle using recursive backtracking.
 *
 * @param board The Sudoku board to solve.
 * @return True if a solution exists, otherwise false.
 */
pub fn solve(board: &mut Board) -> bool {
    let Some((row, col)) = find_empty(board) else {
        return true;
    };

    for value in 1..=9 {
        if is_valid(board, row, col, value) {
            board[row][col] = value;

            if solve(board) {
                return true;
            }

            board[row][col] = 0;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solved_test1() -> Board {
        [
            [1, 4, 3, 9, 8, 6, 2, 5, 7],
            [6, 7, 9, 4, 2, 5, 3, 8, 1],
            [2, 8, 5, 7, 3, 1, 6, 9, 4],
            [9, 6, 2, 3, 5, 4, 1, 7, 8],
            [3, 5, 7, 6, 1, 8, 9, 4, 2],
            [4, 1, 8, 2, 7, 9, 5, 6, 3],
            [8, 2, 1, 5, 6, 7, 4, 3, 9],
            [7, 9, 6, 1, 4, 3, 8, 2, 5],
            [5, 3, 4, 8, 9, 2, 7, 1, 6],
        ]
    }

    fn solved_test2() -> Board {
        [
            [8, 4, 3, 5, 2, 9, 7, 1, 6],
            [9, 7, 6, 3, 1, 8, 5, 2, 4],
            [1, 5, 2, 7, 6, 4, 3, 9, 8],
            [3, 1, 5, 2, 4, 6, 9, 8, 7],
            [7, 8, 9, 1, 3, 5, 6, 4, 2],
            [2, 6, 4, 8, 9, 7, 1, 3, 5],
            [6, 9, 8, 4, 7, 3, 2, 5, 1],
            [4, 3, 1, 6, 5, 2, 8, 7, 9],
            [5, 2, 7, 9, 8, 1, 4, 6, 3],
        ]
    }

    #[test]
    fn test_sudoku_test1_solves_correctly() {
        let mut board = read_puzzle("txt/sudoku-test1.txt");
        let did_solve = solve(&mut board);

        assert!(did_solve);
        assert_eq!(board, solved_test1());
    }

    #[test]
    fn test_sudoku_test2_solves_correctly() {
        let mut board = read_puzzle("txt/sudoku-test2.txt");
        let did_solve = solve(&mut board);

        assert!(did_solve);
        assert_eq!(board, solved_test2());
    }

    #[test]
    fn test_impossible_puzzle_returns_false() {
        let mut board = read_puzzle("txt/sudoku-impossible.txt");
        let did_solve = solve(&mut board);

        assert!(!did_solve);
    }
}