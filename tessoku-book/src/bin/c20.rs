#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]

use proconio::input;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::io::Write;
use std::time::{Duration, Instant};
use rand::Rng;
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
        // eprintln!("adj: {:?}", adj);

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
    fn calc_score(&self, input: &Input) -> f64 {
        let mut pop = vec![0; input.L + 1];  // 1-indexed
        let mut staff = vec![0; input.L + 1];
        for i in 0..input.K {
            let g = self.alloc[i];
            pop[g] += input.A[i];
            staff[g] += input.B[i];
        }
        let p_min = pop.iter().skip(1).min().unwrap();
        let p_max = pop.iter().skip(1).max().unwrap();
        let q_min = staff.iter().skip(1).min().unwrap();
        let q_max = staff.iter().skip(1).max().unwrap();
        let ratio = (*p_min as f64 / *p_max as f64).min(*q_min as f64 / *q_max as f64);
        let all_connected = (1..=input.L).all(|g| check_connectivity(self, input, g));
        if all_connected {
            1e6 * ratio
        } else {
            1e3 * ratio
        }
    }

    #[inline(always)]
    fn print_allocations(&self) {
        for &a in &self.alloc {
            println!("{}", a);
        }
    }
}

fn check_connectivity(state: &State, input: &Input, group: usize) -> bool {
    let nodes: Vec<usize> = (0..input.K).filter(|&i| state.alloc[i] == group).collect();
    if nodes.is_empty() {
        return false;
    }
    let start = nodes[0];
    let mut visited = vec![false; input.K];
    let mut queue = VecDeque::new();
    visited[start] = true;
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for &nb in &input.adj[v] {
            if state.alloc[nb] == group && !visited[nb] {
                visited[nb] = true;
                queue.push_back(nb);
            }
        }
    }
    nodes.into_iter().all(|i| visited[i])
}

/***********************************************************
* Solution
************************************************************/
fn gen_initial_state(input: &Input) -> State {
    // 各地区 i について「平均値との差の絶対値和」を指標にする。指標が小さいほど全体平均に近い
    let mut diff_measure: Vec<(usize, isize)> = (0..input.K)
        .map(|i| {
            let dpop = input.diff_population[i].abs();
            let dstaff = input.diff_staff[i].abs();
            (i, dpop + dstaff)
        })
        .collect();
    diff_measure.sort_by_key(|&(_, diff)| diff);
    // 上位 L 個をシードとして採用（各シードには特別区番号 1～L を割り当て）
    let seed_indices: Vec<usize> = diff_measure.iter().take(input.L).map(|&(i, _)| i).collect();
    let mut assign = vec![usize::MAX; input.K];
    let mut queue = VecDeque::new();
    for (k, &i) in seed_indices.iter().enumerate() {
        assign[i] = k + 1;
        queue.push_back(i);
    }
    // マルチソースBFSで連結部分へ同じ特別区番号を伝播
    while let Some(v) = queue.pop_front() {
        let group = assign[v];
        for &nb in &input.adj[v] {
            if assign[nb] == usize::MAX {
                assign[nb] = group;
                queue.push_back(nb);
            }
        }
    }
    State::new(assign)
}

fn annealing(input: &Input, initial_state: &State) -> State { // 一旦山登り
    let mut rng = rand_pcg::Pcg64Mcg::new(42);
    let mut current = initial_state.clone();
    let mut current_score = current.calc_score(input);

    for _ in 0..100000 {  // TODO
        // ランダムに1地区を選び、その割り当てを1～Lの中から別の値に変更
        let i = rng.gen_range(0..input.K);
        let cur_group = current.alloc[i];
        let mut new_group = cur_group;
        while new_group == cur_group {
            new_group = rng.gen_range(1..input.L + 1);
        }
        let mut candidate = current.clone();
        candidate.alloc[i] = new_group;
        // 変更対象の2グループ（旧グループ、新グループ）が連結であるかチェック
        if !check_connectivity(&candidate, input, cur_group)
            || !check_connectivity(&candidate, input, new_group)
        {
            continue;
        }
        let cand_score = candidate.calc_score(input);
        // 改善していれば採用
        if cand_score > current_score {
            current = candidate;
            current_score = cand_score;
        }
    }
    current
}

fn main() {
    let input = Input::read_input();

    let mut initial_state = gen_initial_state(&input);
    let mut state = annealing(&input, &initial_state);
    state.print_allocations();
    eprintln!("score: {}", initial_state.calc_score(&input));
    eprintln!("score: {}", state.calc_score(&input));
    // let mut time_keeper = TimeKeeper::new(Duration::from_millis(1950), END_TURN);
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {}
