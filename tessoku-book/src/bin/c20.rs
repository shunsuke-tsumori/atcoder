#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]

use proconio::input;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
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
    fn advance(&mut self) {}

    #[inline(always)]
    fn calc_score(&self, input: &Input) -> f64 {
        let mut pop = vec![0; input.L + 1]; // 1-indexed
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
    let trials = 50;
    let mut best_state: Option<State> = None;
    let mut best_score = -1e18;
    for seed in 0..trials {
        let state = gen_init_with_seed(input, seed);
        let score = state.calc_score(input);
        if score > best_score {
            best_score = score;
            best_state = Some(state);
        }
    }
    best_state.unwrap()
}

fn gen_init_with_seed(input: &Input, seed: u64) -> State {
    let mut rng = rand_pcg::Pcg64Mcg::new(seed as u128);
    let K = input.K;
    let L = input.L;

    let mut alloc: Vec<Option<usize>> = vec![None; K];

    // 各グループの目標値（各グループが狙うべき人口・職員数）は、全体をグループ数で割った値とする
    let total_population: usize = input.A.iter().sum();
    let total_staff: usize = input.B.iter().sum();
    let target_pop = total_population / L;
    let target_staff = total_staff / L;

    // 各グループ（1～L）の現在の合計値（1-indexed）
    let mut group_pop = vec![0usize; L + 1];
    let mut group_staff = vec![0usize; L + 1];

    // ランダムにL個の地区を各グループのシードとして割り当てる
    let mut indices: Vec<usize> = (0..K).collect();
    indices.shuffle(&mut rng);
    for g in 1..=L {
        let i = indices[g - 1];
        alloc[i] = Some(g);
        group_pop[g] += input.A[i];
        group_staff[g] += input.B[i];
    }
    let mut assigned_count = L;

    let calc_error = |pop: usize, staff: usize| -> isize {
        let diff_pop = target_pop as isize - pop as isize;
        let diff_staff = target_staff as isize - staff as isize;
        diff_pop * diff_pop + diff_staff * diff_staff
    };

    // 貪欲に既に割当済みの地区の隣接から未割当地区を選び、最も局所スコアが改善される候補を追加
    while assigned_count < K {
        let mut best_gain = std::isize::MIN;
        let mut best_candidate: Option<(usize, usize)> = None; // (district, group)
        for i in 0..K {
            if let Some(g) = alloc[i] {
                for &nb in &input.adj[i] {
                    if alloc[nb].is_none() {
                        let old_pop = group_pop[g];
                        let old_staff = group_staff[g];
                        let old_score = -calc_error(old_pop, old_staff);
                        let new_pop = old_pop + input.A[nb];
                        let new_staff = old_staff + input.B[nb];
                        let new_score = -calc_error(new_pop, new_staff);
                        let gain = new_score - old_score;
                        if gain > best_gain {
                            best_gain = gain;
                            best_candidate = Some((nb, g));
                        }
                    }
                }
            }
        }
        // 隣接からの候補が見つからなければ、ランダムに未割当の地区を選び、各グループへの追加効果が最も良いグループを選ぶ
        if best_candidate.is_none() {
            let unassigned: Vec<usize> = (0..K).filter(|&i| alloc[i].is_none()).collect();
            if let Some(&i) = unassigned.choose(&mut rng) {
                let mut best_local = std::isize::MAX;
                let mut chosen_group = 1;
                for g in 1..=L {
                    let new_pop = group_pop[g] + input.A[i];
                    let new_staff = group_staff[g] + input.B[i];
                    let local = calc_error(new_pop, new_staff);
                    if local < best_local {
                        best_local = local;
                        chosen_group = g;
                    }
                }
                best_candidate = Some((i, chosen_group));
            }
        }
        if let Some((district, group)) = best_candidate {
            alloc[district] = Some(group);
            group_pop[group] += input.A[district];
            group_staff[group] += input.B[district];
            assigned_count += 1;
        }
    }
    let final_alloc = alloc.into_iter().map(|x| x.unwrap()).collect();
    State::new(final_alloc)
}

fn annealing(input: &Input, initial_state: &State, time_budget: Duration) -> State {
    let mut rng = rand_pcg::Pcg64Mcg::new(42);
    let mut current = initial_state.clone();
    let mut current_score = current.calc_score(input);
    let start_time = Instant::now();
    let total_time = time_budget.as_secs_f64();

    let T0: f64 = 1e5;
    let T1: f64 = 1e2;
    let mut current_T = T0;

    let mut iter: usize = 0;
    let mut accepted_moves = 0;
    let mut worsened_moves = 0;

    loop {
        iter += 1;
        if (iter & ((1 << 10) - 1)) == 0 {
            if start_time.elapsed() >= time_budget {
                break;
            }
            // 経過時間に応じて指数的に温度を下げる
            let elapsed = start_time.elapsed().as_secs_f64();
            let ratio = elapsed / total_time;
            current_T = T0 * (T1 / T0).powf(ratio);
        }

        #[cfg(debug_assertions)]
        if iter % 1000 == 0 {
            eprintln!(
                "Iter {}: Score = {:.2}, T = {:.2}, accepted = {}, worsened = {}",
                iter, current_score, current_T, accepted_moves, worsened_moves
            );
        }

        // ランダムに1地区を選び、新グループに変更
        let i = rng.gen_range(0..input.K);
        let cur_group = current.alloc[i];
        let mut new_group = cur_group;
        while new_group == cur_group {
            new_group = rng.gen_range(1..(input.L + 1));
        }
        let mut candidate = current.clone();
        candidate.alloc[i] = new_group;

        // 変更対象の旧グループと新グループが連結かチェック
        if !check_connectivity(&candidate, input, cur_group)
            || !check_connectivity(&candidate, input, new_group)
        {
            continue;
        }
        let cand_score = candidate.calc_score(input);
        let delta = cand_score - current_score;

        // 改善していれば必ず採用、悪化なら温度に応じた確率で採用
        if delta > 0.0 || rng.gen::<f64>() < (delta / current_T).exp() {
            current = candidate;
            current_score = cand_score;
            accepted_moves += 1;
            if delta < 0.0 {
                worsened_moves += 1;
            }
        }
    }
    current
}

fn main() {
    let input = Input::read_input();

    let mut initial_state = gen_initial_state(&input);
    let mut state = annealing(&input, &initial_state, Duration::from_millis(930));
    state.print_allocations();
    eprintln!("score: {}", initial_state.calc_score(&input));
    eprintln!("score: {}", state.calc_score(&input));
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {}
