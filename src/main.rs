/*
 * Name: Ann Mathew
 * Email: ann.e.mathew@vanderbilt.edu
 * Project: CS 3270 Project 3 Rust Sudoku Solver
 */

mod solver;

use std::io;

use solver::{print_board, read_puzzle, solve};

/**
 * Prompts the user for a puzzle filename.
 *
 * If the user presses Enter without typing a file name, the default
 * file name sudoku-test1.txt is used from the txt folder.
 *
 * @return The full path to the puzzle file.
 */
fn prompt_for_filename() -> String {
    let mut input = String::new();

    println!(
        "Enter the puzzle filename located in the txt folder \
(example: sudoku-test1.txt)."
    );
    println!("Press Enter to use sudoku-test1.txt:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let trimmed = input.trim();

    if trimmed.is_empty() {
        "txt/sudoku-test1.txt".to_string()
    } else {
        format!("txt/{}", trimmed)
    }
}

/**
 * Runs the Sudoku solver program.
 */
fn main() {
    let file_path = prompt_for_filename();
    let mut board = read_puzzle(&file_path);

    println!("\nInitial puzzle:");
    print_board(&board);

    if solve(&mut board) {
        println!("\nSolved puzzle:");
        print_board(&board);
    } else {
        println!("\nThis puzzle has no solution.");
    }
}