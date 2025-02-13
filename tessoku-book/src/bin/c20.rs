#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]

use proconio::input;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

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
    group_pop: Vec<usize>,
    group_staff: Vec<usize>,
    score: f64,
}

impl State {
    #[inline(always)]
    fn new(alloc: Vec<usize>, group_pop: Vec<usize>, group_staff: Vec<usize>, score: f64) -> Self {
        Self {
            alloc,
            group_pop,
            group_staff,
            score,
        }
    }

    #[inline(always)]
    fn print_allocations(&self) {
        for &a in &self.alloc {
            println!("{}", a);
        }
    }
}

// 移動変更の履歴（ロールバック用）
struct MoveRecord {
    district: usize,
    old_group: usize,
    new_group: usize,
    old_score: f64,
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

// 状態の group_pop, group_staff から比率を再計算する
fn update_score(state: &mut State, input: &Input) -> f64 {
    let p_min = state.group_pop.iter().skip(1).min().unwrap();
    let p_max = state.group_pop.iter().skip(1).max().unwrap();
    let q_min = state.group_staff.iter().skip(1).min().unwrap();
    let q_max = state.group_staff.iter().skip(1).max().unwrap();
    let ratio = (*p_min as f64 / *p_max as f64).min(*q_min as f64 / *q_max as f64);
    let all_connected = (1..=input.L).all(|g| check_connectivity(state, input, g));
    let multiplier = if all_connected { 1e6 } else { 1e3 };
    let new_score = multiplier * ratio;
    state.score = new_score;
    new_score
}

/// 移動を適用（in-place 更新）し、その変更内容を MoveRecord として返す
fn apply_move(state: &mut State, district: usize, new_group: usize, input: &Input) -> MoveRecord {
    let old_group = state.alloc[district];
    let old_score = state.score;
    // 状態の更新
    state.alloc[district] = new_group;
    state.group_pop[old_group] -= input.A[district];
    state.group_staff[old_group] -= input.B[district];
    state.group_pop[new_group] += input.A[district];
    state.group_staff[new_group] += input.B[district];
    MoveRecord {
        district,
        old_group,
        new_group,
        old_score,
    }
}

/// apply_move での変更を元に戻す
fn rollback_move(state: &mut State, record: &MoveRecord, input: &Input) {
    let d = record.district;
    let old_group = record.old_group;
    let new_group = record.new_group;
    state.alloc[d] = old_group;
    state.group_pop[new_group] -= input.A[d];
    state.group_staff[new_group] -= input.B[d];
    state.group_pop[old_group] += input.A[d];
    state.group_staff[old_group] += input.B[d];
    state.score = record.old_score;
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
        let score = state.score;
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

    // ランダムに L 個の地区を各グループのシードとして割り当てる
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

    // 貪欲に、既に割当済みの地区の隣接から未割当地区を選び、局所的なスコア改善が大きい候補を追加する
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
        // 隣接からの候補がなければ、ランダムに未割当の地区を選び、最も適したグループに割り当てる
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
    let mut state = State::new(final_alloc, group_pop, group_staff, 0.0);
    update_score(&mut state, input);
    state
}

fn annealing(input: &Input, initial_state: &State, time_budget: Duration) -> State {
    let mut rng = rand_pcg::Pcg64Mcg::new(42);
    let mut current = initial_state.clone();
    let mut current_score = current.score;
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

        let record = apply_move(&mut current, i, new_group, input);

        // 変更対象の旧グループと新グループの連結性をチェック
        if !check_connectivity(&current, input, cur_group)
            || !check_connectivity(&current, input, new_group)
        {
            rollback_move(&mut current, &record, input);
            continue;
        }

        // 集計情報はすでにインクリメンタル更新済みなので、スコア再計算
        let new_score = update_score(&mut current, input);
        let delta = new_score - record.old_score;

        // 改善なら必ず採用、悪化なら確率的に採用
        if delta > 0.0 || rng.gen::<f64>() < (delta / current_T).exp() {
            current_score = new_score;
            accepted_moves += 1;
            if delta < 0.0 {
                worsened_moves += 1;
            }
        } else {
            rollback_move(&mut current, &record, input);
        }
    }
    current
}

fn main() {
    let input = Input::read_input();

    let initial_state = gen_initial_state(&input);
    let state = annealing(&input, &initial_state, Duration::from_millis(930));
    state.print_allocations();
    eprintln!("initial score: {}", initial_state.score);
    eprintln!("final score: {}", state.score);
}

/***********************************************************
* Tests
************************************************************/
#[cfg(test)]
mod tests {}
