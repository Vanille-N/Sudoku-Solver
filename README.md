# Sudoku Solver

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A multithreaded sudoku solver written in Rust.

Uses a simple backtracking algorithm.

The main purpose of this project was to experiment with multithreading. Benchmarks and various attempts are available inside `src/main.rs`.

Takes input as
```
..3.2.6..9..3.5..1..18.64....81.29..7.......8..67.82....26.95..8..2.3..9..5.1.3..
```

And solves puzzle as
```
Input n°0
. . 3 . 2 . 6 . .
9 . . 3 . 5 . . 1
. . 1 8 . 6 4 . .
. . 8 1 . 2 9 . .
7 . . . . . . . 8
. . 6 7 . 8 2 . .
. . 2 6 . 9 5 . .
8 . . 2 . 3 . . 9
. . 5 . 1 . 3 . .

Input n°0
Solved, 151 paths explored
4 8 3 9 2 1 6 5 7
9 6 7 3 4 5 8 2 1
2 5 1 8 7 6 4 9 3
5 4 8 1 3 2 9 7 6
7 2 9 5 6 4 1 3 8
1 3 6 7 9 8 2 4 5
3 7 2 6 8 9 5 1 4
8 1 4 2 5 3 7 6 9
6 9 5 4 1 7 3 8 2
```
