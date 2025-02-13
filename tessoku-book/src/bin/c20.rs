#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::input;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use std::io::Write;
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
    K: usize,
    L: usize,
    A: Vec<usize>,
    B: Vec<usize>,
    C: Vec<Vec<usize>>,
    average_population: usize,
    average_staff: usize,
    diff_population: Vec<isize>,
    diff_staff: Vec<isize>,
    adj: Vec<Vec<usize>>,
}

impl Input {
    #[inline(always)]
    fn read_input() -> Self {
        input! {
            N: usize,
            K: usize,
            L: usize,
            AB: [(usize, usize); K],
            C: [[usize; N]; N],
        }
        let A: Vec<usize> = AB.iter().map(|&(a, _)| a).collect();
        let B: Vec<usize> = AB.iter().map(|&(_, b)| b).collect();

        let total_population: usize = A.iter().sum();
        let total_staff: usize = B.iter().sum();
        let average_population = total_population / K;
        let average_staff = total_staff / K;

        let diff_population: Vec<isize> = A
            .iter()
            .map(|&a| a as isize - average_population as isize)
            .collect();
        let diff_staff: Vec<isize> = B
            .iter()
            .map(|&b| b as isize - average_staff as isize)
            .collect();

        let adj = Self::gen_adj(&C, N, K);
        eprintln!("adj: {:?}", adj);

        Self {
            N,
            K,
            L,
            A,
            B,
            C,
            average_population,
            average_staff,
            diff_population,
            diff_staff,
            adj,
        }
    }

    // 隣接地区を生成する
    fn gen_adj(raw_map: &Vec<Vec<usize>>, N: usize, district_count: usize) -> Vec<Vec<usize>> {
        let mut adj = vec![vec![]; district_count];
        for row in 0..N {
            for col in 0..N {
                let d = raw_map[row][col].wrapping_sub(1);
                if d >= district_count {
                    continue;
                }
                for &(dr, dc) in &[(0, 1), (0, !0), (1, 0), (!0, 0)] {
                    let nr = row.wrapping_add(dr);
                    let nc = col.wrapping_add(dc);
                    if nr < N && nc < N {
                        let nd = raw_map[nr][nc].wrapping_sub(1);
                        if nd < district_count && nd != d {
                            adj[d].push(nd);
                        }
                    }
                }
            }
        }
        for v in &mut adj {
            v.sort_unstable();
            v.dedup();
        }
        adj
    }
}

/***********************************************************
* State
************************************************************/
#[derive(Debug, Clone)]
struct State {
    alloc: Vec<usize>,
}

impl State {
    #[inline(always)]
    fn new(alloc: Vec<usize>) -> Self {
        Self { alloc }
    }

    #[inline(always)]
    fn is_done(&self) -> bool {
        true
    }

    #[inline(always)]
    fn advance(&mut self) {}

    #[inline(always)]
    fn calc_score(&self) -> f64 {
        0.0
    }

    #[inline(always)]
    fn print_allocations(&self) {
        for &a in &self.alloc {
            println!("{}", a);
        }
    }
}

/***********************************************************
* Solution
************************************************************/
fn gen_initial_state(input: &Input) -> State {
    let mut rng = Pcg64Mcg::new(42);
    let mut alloc: Vec<usize> = Vec::with_capacity(input.K);
    for i in 1..=input.L {
        alloc.push(i);
    }
    let uniform = Uniform::from(1..=input.L);
    for _ in input.L..input.K {
        alloc.push(uniform.sample(&mut rng));
    }
    alloc.shuffle(&mut rng);
    State::new(alloc)
}

fn annealing(input: &Input, initial_state: &State) -> State {
    initial_state.clone()
}

fn main() {
    let input = Input::read_input();

    let mut initial_state = gen_initial_state(&input);
    let mut state = annealing(&input, &initial_state);
    // state.print_allocations();
    // let mut time_keeper = TimeKeeper::new(Duration::from_millis(1950), END_TURN);
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {}
