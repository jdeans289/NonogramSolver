# NonogramSolver

A solver for Nonogram puzzles implemented in Rust with a SAT solver.

Just use `cargo run` to run!

Assumes a square puzzle. This is not a limitation of the algorithm, only this implementation.

Puzzle files are in the `puzzles/` directory.

Input format:

The first row is the dimension=n of the square puzzle. A 5 means 5x5
The next n rows are the rules for each column, left to right.
The next n rows are the rules for each row, top to bottom.

A rule is a space separated list of numbers for the size of the sets in each row/col.

## Example

Input:
```
5
3 1
1 1 1
1 1 1
1 1 1
1 3
5
1
5
1
5
```

Pretty printed solution:
```
    1 1 1
  3 1 1 1 1
  1 1 1 1 3
5 x x x x x
1 x
5 x x x x x
1         x
5 x x x x x
```