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
    N: usize,
    coordinates: Vec<(i64, i64)>,
}

impl Input {
    #[inline(always)]
    fn read_input() -> Self {
        input! {
            N: usize,
            coordinates: [(i64, i64); N],
        }
        Self { N, coordinates }
    }
}

/***********************************************************
* State
************************************************************/
#[derive(Debug, Clone)]
struct State {
    visited: Vec<bool>,
    visited_order: Vec<usize>, // 0-indexed で格納する
}

impl State {
    #[inline(always)]
    fn new(N: usize) -> Self {
        let mut visited = vec![false; N];
        visited[0] = true;
        Self {
            visited,
            visited_order: vec![0],
        }
    }

    #[inline(always)]
    fn is_done(&self) -> bool {
        true
    }

    #[inline(always)]
    fn advance(&mut self, next_index: usize) {
        self.visited[next_index] = true;
        self.visited_order.push(next_index);
    }

    #[inline(always)]
    fn update(&mut self) {}

    #[inline(always)]
    fn calc_score(&self, coordinates: Vec<(i64, i64)>) -> f64 {
        let length = self.visited_order.len();
        let mut score = 0.0;
        for i in 0..length {
            let x1 = coordinates[self.visited_order[i]];
            let x2 = coordinates[self.visited_order[(i + 1) % length]];
            score += dist(x1, x2)
        }
        score
    }

    #[inline(always)]
    fn print_order(&self) {
        let length = self.visited_order.len();
        for i in 0..length {
            let x = self.visited_order[i];
            println!("{}", x + 1);
        }
    }
}

/***********************************************************
* Solution
************************************************************/
fn dist_pow2(x1: (i64, i64), x2: (i64, i64)) -> i64 {
    (x1.0 - x2.0).pow(2) + (x1.1 - x2.1).pow(2)
}

fn dist(x1: (i64, i64), x2: (i64, i64)) -> f64 {
    (dist_pow2(x1, x2) as f64).sqrt()
}

fn greedy_solution(input: Input) -> State {
    let mut state = State::new(input.N);
    for i in 1..input.N {
        state.advance(i);
    }
    state.advance(0);
    state
}

fn main() {
    let input = Input::read_input();

    let initial_solution = greedy_solution(input);

    initial_solution.print_order();

    // let mut state = State::new(input.N);
    // let mut time_keeper = TimeKeeper::new(Duration::from_millis(1950), END_TURN);
    //
    // for t in 0..END_TURN {
    //     time_keeper.set_turn(t);
    //     state.update();
    // }
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {
    use crate::dist;
    use crate::dist_pow2;

    #[test]
    fn it_works() {
        assert_eq!(dist_pow2((0, 0), (1000, 1000)), 2000000);
        assert_eq!(dist_pow2((1000, 0), (0, 1000)), 2000000);
        assert_eq!(dist((1000, 0), (0, 1000)), 1414.213562373095);
    }
}
