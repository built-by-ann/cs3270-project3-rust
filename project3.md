# CS 3270 – Project 3

## Name
Ann Mathew

## Email
ann.e.mathew@vanderbilt.edu

## Language
Rust

---

## How to Run the Program

From the root directory of the project (the folder with `Cargo.toml`), run:

```bash
cargo run
```

The program will ask you for the name of a puzzle file.

All puzzle files are in the `txt` folder, but you should only type the filename (do not include `txt/`).

**Example:**
```
sudoku-test1.txt
```

If you just press Enter without typing anything, it will default to:
```
sudoku-test1.txt
```

### What the program does:
- Prints the given puzzle  
- Tries to solve it  
- Prints the solved puzzle if a solution exists  

If the puzzle can’t be solved, it will print:
```
This puzzle has no solution.
```

---

## How to Run the Tests

From the root directory, run:

```bash
cargo test
```

This runs all test cases using Rust’s built-in testing.

---

## Files Included

- `src/main.rs` – handles user input and runs the program  
- `src/solver.rs` – has the Sudoku solving logic and tests  
- `txt/sudoku-test1.txt` – unsolved puzzle 1 (solvable)
- `txt/sudoku-test2.txt` – unsolved puzzle 2 (solvable)
- `txt/sudoku-impossible.txt` – unsolved puzzle 3 (unsolvable)
- `.github/workflows/rust.yml` – GitHub Actions setup  
- `Cargo.toml` – project configuration  

---

## Solver Approach

The solver uses a recursive backtracking algorithm.

### How it works:
1. Find an empty cell (represented by `0`)  
2. Try numbers `1` through `9`  
3. Check if the number is valid:
   - Not already in the same row  
   - Not already in the same column  
   - Not already in the same 3×3 box  
4. If it works, place the number and keep going  
5. If it leads to a dead end, backtrack and try the next number  

If all cells get filled, the puzzle is solved. If not, there is no solution.

---

## Testing

There are three test cases:

- `sudoku-test1.txt` → checked against the correct solution  
- `sudoku-test2.txt` → checked against the correct solution  
- `sudoku-impossible.txt` → confirmed to have no solution  

The tests work by comparing the full solved board to the expected result.

---

## GitHub Actions

GitHub Actions is set up to automatically run:

```bash
cargo test
```

on every push and pull request.

If everything passes, you can see the green checkmark.

---
