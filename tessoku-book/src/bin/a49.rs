#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use once_cell::sync::Lazy;
use proconio::input;
use proconio::source::line::LineSource;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand_pcg::Pcg64Mcg;
use std::collections::VecDeque;
use std::io::{stdin, stdout, BufReader, Write};
use std::time::{Duration, Instant};

/***********************************************************
* Consts
************************************************************/
const END_TURN: usize = 100;

/***********************************************************
* Macros
************************************************************/
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

/***********************************************************
* TimeKeeper
************************************************************/
struct TimeKeeper {
    start_time: Instant,
    before_time: Instant,
    time_threshold: Duration,
    end_turn: usize,
    turn: usize,
}

impl TimeKeeper {
    fn new(time_threshold: Duration, end_turn: usize) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            before_time: now,
            time_threshold,
            end_turn,
            turn: 0,
        }
    }

    #[inline(always)]
    fn set_turn(&mut self, t: usize) {
        self.turn = t;
        self.before_time = Instant::now();
    }

    #[inline(always)]
    fn is_time_over(&self) -> bool {
        let now = Instant::now();
        let whole_ms = now.duration_since(self.start_time).as_millis();
        let last_ms = now.duration_since(self.before_time).as_millis();
        let remaining_time = self.time_threshold.as_millis().saturating_sub(whole_ms);
        let remaining_turns = self.end_turn - self.turn;
        if remaining_turns == 0 {
            false
        } else {
            let now_threshold = remaining_time / (remaining_turns as u128);
            last_ms >= now_threshold
        }
    }
}

/***********************************************************
* Input
************************************************************/
#[derive(Debug, Clone)]
struct Input {
    T: usize,
    P: Vec<usize>,
    Q: Vec<usize>,
    R: Vec<usize>,
}

impl Input {
    #[inline(always)]
    fn read_input() -> Self {
        input! {
            T: usize,
            PQR: [(usize, usize, usize); T],
        }
        let mut P = Vec::with_capacity(T);
        let mut Q = Vec::with_capacity(T);
        let mut R = Vec::with_capacity(T);
        for (p, q, r) in PQR {
            P.push(p - 1);
            Q.push(q - 1);
            R.push(r - 1);
        }
        Self { T, P, Q, R }
    }
}

/***********************************************************
* State
************************************************************/
#[derive(Debug, Clone)]
struct State {
    x: [i8; 20],
    turn: u16,
    history: Vec<bool>
}

impl State {
    #[inline(always)]
    fn new(input: Input) -> Self {
        Self {
            x: [0; 20],
            turn: 0,
            history: Vec::new()
        }
    }

    #[inline(always)]
    fn is_done(&self) -> bool {
        true
    }

    #[inline(always)]
    fn advance(&mut self) {}

    #[inline(always)]
    fn calc_score(&self) -> usize {
        self.x.iter().filter(|&&x| x == 0).count()
    }
}

/***********************************************************
* Solution
************************************************************/
fn calc() -> i32 {
    1
}

fn main() {
    let input = Input::read_input();

    let mut state = State::new(input);
    let mut time_keeper = TimeKeeper::new(Duration::from_millis(1950), END_TURN);
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn it_works() {
        assert_eq!(calc(), 1);
    }
}
