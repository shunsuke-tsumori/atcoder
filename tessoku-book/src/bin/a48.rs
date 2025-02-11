#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::input;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand_pcg::Pcg64Mcg;
use std::io::Write;
use std::time::{Duration, Instant};

/***********************************************************
* Consts
************************************************************/

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
    time_threshold: Duration,
}

impl TimeKeeper {
    fn new(time_threshold: Duration) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            time_threshold,
        }
    }

    #[inline(always)]
    fn is_time_over(&self) -> bool {
        let now = Instant::now();
        now.duration_since(self.start_time) >= self.time_threshold
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
    fn is_done(&self, N: usize) -> bool {
        self.visited_order.len() >= N
    }

    #[inline(always)]
    fn advance(&mut self, next_index: usize) {
        self.visited[next_index] = true;
        self.visited_order.push(next_index);
    }

    #[inline(always)]
    fn is_visitable(&self, next_index: usize) -> bool {
        !self.visited[next_index]
    }

    #[inline(always)]
    fn dist_to_next(&self, next_index: usize, coordinates: &Vec<(i64, i64)>) -> f64 {
        let current_coordinate = coordinates[self.visited_order[self.visited_order.len() - 1]];
        let next_coordinate = coordinates[next_index];
        dist(current_coordinate, next_coordinate)
    }

    #[inline(always)]
    fn calc_score(&self, coordinates: &Vec<(i64, i64)>) -> f64 {
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

fn greedy_solution(input: &Input) -> State {
    let mut state = State::new(input.N);

    while !state.is_done(input.N) {
        let mut min_score = std::f64::MAX;
        let mut min_index = 0;
        for i in 0..input.N {
            if state.is_visitable(i) {
                let score = state.dist_to_next(i, &input.coordinates);
                if score < min_score {
                    min_score = score;
                    min_index = i;
                }
            }
        }
        state.advance(min_index);
    }

    // 最後はスタートに戻る
    state.advance(0);
    state
}

fn two_opt(state: State, input: &Input) -> State {
    let mut rng = Pcg64Mcg::new(42);
    let mut time_keeper = TimeKeeper::new(Duration::from_millis(990));

    let mut route = state.visited_order.clone();
    let mut turn = 0;

    while turn % 100 != 0 || !time_keeper.is_time_over() {
        let l_range = Uniform::from(1..=input.N - 2);
        let l = l_range.sample(&mut rng);
        let r_range = Uniform::from((l + 1)..=input.N - 1);
        let r = r_range.sample(&mut rng);

        // 2-opt swap の影響は，区間[l-1,l] と [r, r+1] の辺が [l-1, r] と [l, r+1] に置換される
        let a = route[l - 1];
        let b = route[l];
        let c = route[r];
        let d = route[r + 1];
        let current_cost = dist(input.coordinates[a], input.coordinates[b])
            + dist(input.coordinates[c], input.coordinates[d]);
        let new_cost = dist(input.coordinates[a], input.coordinates[c])
            + dist(input.coordinates[b], input.coordinates[d]);
        let delta = new_cost - current_cost;
        if delta < 0.0 {
            // 改善なら区間 [l, r] を反転
            route[l..=r].reverse();
        }
        turn += 1;
    }
    eprintln!("turn: {}", turn);
    State {
        visited: vec![true; input.N],
        visited_order: route,
    }
}

fn main() {
    let input = Input::read_input();
    let initial_solution = greedy_solution(&input);
    let optimized_solution = two_opt(initial_solution, &input);
    optimized_solution.print_order();
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
