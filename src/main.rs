extern crate scoped_threadpool;
extern crate rayon;

use std::io::{self, BufRead};
use std::fmt;
use std::ops;
use scoped_threadpool::Pool;
use rayon::prelude::*;

enum State {
    Given(u8),
    Guess(u8),
    Blank,
}

struct Grid {
    grid: Vec<State>,
    count: u32,
    rowcheck: [u16; 9],
    colcheck: [u16; 9],
    sqcheck: [u16; 9],
}

#[derive(PartialEq)]
enum ReadStatus {
    Ok,
    TooShort,
    InvalidChr,
}

#[derive(PartialEq)]
enum SolveStatus {
    Unknown,
    Solved,
    Failed,
}

#[derive(PartialEq)]
enum Action {
    Advance,
    Backtrace,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: Vec::<State>::new(),
            count: 0,
            colcheck: [0; 9],
            rowcheck: [0; 9],
            sqcheck: [0; 9],
        }
    }

    pub fn parse(&mut self, str: String) -> ReadStatus {
        self.count = 0;
        self.colcheck = [0; 9];
        self.rowcheck = [0; 9];
        self.sqcheck = [0; 9];
        let mut res = ReadStatus::Ok;
        let s = str.chars().collect::<Vec<_>>();
        if self.grid.len() == 81 {
            for k in 0..81 {
                if k < s.len() {
                     let n = match s[k] {
                        '_' | '.' | '0' => {
                            State::Blank
                        }
                        c @ '1'..='9' => {
                            let n = c as u8 - '0' as u8;
                            self.forbid(n, k);
                            State::Given(n)
                        }
                        _ => {
                            res = ReadStatus::InvalidChr;
                            State::Blank
                        }
                    };
                    self.grid[k] = n ;
                } else {
                    res = ReadStatus::TooShort;
                    self.grid[k] = State::Blank;
                }
            }
        } else {
            self.grid = Vec::<State>::new();
            for k in 0..81 {
                if k < s.len() {
                    let n = match s[k] {
                        '_' | '.' | '0' => {
                            State::Blank
                        }
                        c @ '1'..='9' => {
                            let n = c as u8 - '0' as u8;
                            self.forbid(n, k);
                            State::Given(n)
                        }
                        _ => {
                            res = ReadStatus::InvalidChr;
                            State::Blank
                        }
                    };
                    self.grid.push(n);
                } else {
                    res = ReadStatus::TooShort;
                    self.grid.push(State::Blank);
                }
            }
        }
        res
    }

    fn check(&self, n: u8, k: usize) -> bool {
        let i0 = k / 9;
        let j0 = k % 9;
        self.rowcheck[i0] & (1<<n) == 0
        && self.colcheck[j0] & (1<<n) == 0
        && self.sqcheck[i0 - i0%3 + j0/3] & (1<<n) == 0
    }

    fn toggle(&mut self, n: u8, k: usize) {
        let i0 = k / 9;
        let j0 = k % 9;
        self.rowcheck[i0] ^= 1<<n;
        self.colcheck[j0] ^= 1<<n;
        self.sqcheck[i0 - i0%3 + j0/3] ^= 1<<n;
    }

    fn forbid(&mut self, n: u8, k: usize) {
        self.toggle(n, k);
    }

    fn allow(&mut self, n: u8, k: usize) {
        self.toggle(n, k);
    }

    fn solve(&mut self) -> SolveStatus {
        let mut idx = 1;
        let mut action = Action::Advance;
        while idx > 0 {
            if idx == 82 {
                return SolveStatus::Solved;
            }
            match self.grid[idx-1] {
                State::Blank => {
                    let mut k = 1;
                    while !self.check(k, idx-1) {
                        k += 1;
                    }
                    if k < 10 {
                        self.grid[idx-1] = State::Guess(k);
                        action = Action::Advance;
                        self.forbid(k, idx-1);
                        idx += 1;
                    } else {
                        idx -= 1;
                        self.count += 1;
                        action = Action::Backtrace;
                    }
                }
                State::Given(_) => {
                    match action {
                        Action::Advance => {
                            idx += 1;
                        }
                        Action::Backtrace => {
                            idx -= 1;
                        }
                    }
                }
                State::Guess(c) => {
                    if action == Action::Backtrace {
                        self.allow(c, idx-1);
                    }
                    let mut k = c+1;
                    while !self.check(k, idx-1) {
                        k += 1;
                    }
                    if k < 10 {
                        self.grid[idx-1] = State::Guess(k);
                        action = Action::Advance;
                        self.forbid(k, idx-1);
                        idx += 1;
                    } else {
                        self.grid[idx-1] = State::Blank;
                        idx -= 1;
                        self.count += 1;
                        action = Action::Backtrace;
                    }
                }
            }
        }
        SolveStatus::Failed
    }
}

impl ops::Index<[usize; 2]> for Grid {
    type Output = State;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.grid[idx[0] * 9 + idx[1]]
    }
}

impl ops::IndexMut<[usize; 2]> for Grid {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut State {
        &mut self.grid[idx[0] * 9 + idx[1]]
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{} ", match self[[i, j]]{
                    State::Blank => '.',
                    State::Guess(c) => ('0' as u8 + c) as char,
                    State::Given(c) => ('0' as u8 + c) as char,
                })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{} ", match self[[i, j]]{
                    State::Blank => '.',
                    State::Guess(c) => ('0' as u8 + c) as char,
                    State::Given(c) => ('0' as u8 + c) as char,
                })?;
            }
        }
        writeln!(f, "")?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

// Without parallelizing

fn main_sequential() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut i = 1;
    let mut g = Grid::new();
    for line in lock.lines() {
        if g.parse(line.unwrap()) == ReadStatus::Ok {
            println!("Input n°{}", i);
            println!("{:?}", g);
            if g.solve() == SolveStatus::Solved {
                println!("Solved, {} paths explored", g.count);
                println!("{:?}", g);
            } else {
                println!("No solution, {} paths explored", g.count);
                println!("{:?}", g);
            }
        }
        i += 1;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

// Sequential again, but all grids are stored in a single vector.

fn main_vec() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut grids: Vec<(SolveStatus, Grid)> = Vec::new();

    for (i, line) in lock.lines().enumerate() {
        let mut g = Grid::new();
        if g.parse(line.unwrap()) == ReadStatus::Ok {
            println!("Input n°{}", i);
            println!("{:?}", g);
            grids.push((SolveStatus::Unknown, g));
        }
    }

    grids.iter_mut().for_each(|g|
        g.0 = g.1.solve()
    );

    for i in 0..grids.len() {
        if grids[i].0 == SolveStatus::Solved {
            println!("Input n°{}", i);
            println!("Solved, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        } else {
            println!("Input n°{}", i);
            println!("No solution, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

// With multithreading

fn main_scoped_pool() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut grids: Vec<(SolveStatus, Grid)> = Vec::new();

    let n_workers: u32 = (grids.len() + 1) as u32;
    let mut pool = Pool::new(n_workers);

    for (i, line) in lock.lines().enumerate() {
        let mut g = Grid::new();
        if g.parse(line.unwrap()) == ReadStatus::Ok {
            println!("Input n°{}", i);
            println!("{:?}", g);
            grids.push((SolveStatus::Unknown, g));
        }
    }
    pool.scoped(|scoped| {
        for g in &mut grids {
            scoped.execute(move || {
                g.0 = g.1.solve();
            });
        }
    });
    for i in 0..grids.len() {
        if grids[i].0 == SolveStatus::Solved {
            println!("Input n°{}", i);
            println!("Solved, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        } else {
            println!("Input n°{}", i);
            println!("No solution, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        }
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

// Let's try dividing the data in 2 to limit the number of threads

fn main_scoped_split() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut grids: Vec<(SolveStatus, Grid)> = Vec::new();

    let n_workers: u32 = (grids.len() + 1) as u32;
    let mut pool = Pool::new(n_workers);

    for (i, line) in lock.lines().enumerate() {
        let mut g = Grid::new();
        if g.parse(line.unwrap()) == ReadStatus::Ok {
            println!("Input n°{}", i);
            println!("{:?}", g);
            grids.push((SolveStatus::Unknown, g));
        }
    }
    let mid = grids.len() / 2;
    let (left, right) = grids.split_at_mut(mid);
    pool.scoped(|scoped| {
        scoped.execute(move || {
            for g in left {
                g.0 = g.1.solve();
                println!("A");
            }
        });
        scoped.execute(move || {
            for g in right {
                g.0 = g.1.solve();
                println!("B");
            }
        });
    });
    println!();
    for i in 0..grids.len() {
        if grids[i].0 == SolveStatus::Solved {
            println!("Input n°{}", i);
            println!("Solved, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        } else {
            println!("Input n°{}", i);
            println!("No solution, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

// More high-level in abstraction: threads are not managed manually

fn main_rayon() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut grids: Vec<(SolveStatus, Grid)> = Vec::new();

    for (i, line) in lock.lines().enumerate() {
        let mut g = Grid::new();
        if g.parse(line.unwrap()) == ReadStatus::Ok {
            println!("Input n°{}", i);
            println!("{:?}", g);
            grids.push((SolveStatus::Unknown, g));
        }
    }

    grids.par_iter_mut().for_each(|g|
        g.0 = g.1.solve()
    );

    for i in 0..grids.len() {
        if grids[i].0 == SolveStatus::Solved {
            println!("Input n°{}", i);
            println!("Solved, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        } else {
            println!("Input n°{}", i);
            println!("No solution, {} paths explored", grids[i].1.count);
            println!("{:?}", grids[i].1);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    //main_sequential();
    //main_vec();
    //main_scoped_pool();
    //main_scoped_split();
    main_rayon();
}

// Warning: running this project with cargo without building causes significant overhead.
// Build before running to enable optimal performance.
// $ cargo run                                                               0m0,749s
// $ RUSTFLAGS="-C target-cpu=native" cargo build                            0m0,052s
// Both are measured for input-x50.
// input-x157 manually multithreaded DNF for input-x157

// We can see that `cargo run` is in fact not the right choice.
// From here onwards, we build with
// $ RUSTFLAGS="-C target-cpu=native" cargo build --release
// And we run with whatever is in `./target/release/deps/`

// With `main_sequential`
// $ [...] < data/input-x50                                                  0m0,057s
// $ [...] < data/input-x157                                                0m13,952s

// With `main_vec`
// $ [...] < data/input-x50                                                  0m0,061s
// $ [...] < data/input-x157                                                0m15,438s

// With `main_scoped_pool`
// $ [...] < data/input-x50                                                  0m0,060s
// $ [...] < data/input-x157                                                0m13,651s

// With `main_scoped_split`
// $ [...] < data/input-x50                                                  0m0,061s
// $ [...] < data/input-x157                                                0m14,503s
// It should be noted that all sudoku in thread B were solved after all those
// in thread A, effectively rendering the multithreading useless.

// With `main_rayon`
// $ [...] < data/input-x50                                                  0m0,037s
// $ [...] < data/input-x157                                                 0m7,394s
// Interesting takeaway: when using rayon, `real` time is 7s, but `user` time is 21s.
// All 4 cores are running for a mere x2 increase in speed !
// With `sudoku-input-short`, everything happens too quickly to be able to confirm that all
// cores are active, but it should be noted that `user` time is twice `real` time at 38ms.
