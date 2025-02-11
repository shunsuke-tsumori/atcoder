#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::input;
use std::cmp::Reverse;

/***********************************************************
* Consts
************************************************************/
const BEAM_WIDTH: usize = 30_000;

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
    history: Vec<bool>,
    score: usize,
}

impl State {
    #[inline(always)]
    fn new() -> Self {
        Self {
            x: [0; 20],
            turn: 0,
            history: Vec::new(),
            score: 0,
        }
    }

    #[inline(always)]
    fn is_done(&self, T: usize) -> bool {
        (self.turn as usize) >= T
    }

    // action が +1 なら操作 A（対象に +1）、-1 なら操作 B（対象に -1）を意味する。
    #[inline(always)]
    fn operate(&self, p: usize, q: usize, r: usize, action: i8) -> Self {
        let mut new_state = self.clone();

        new_state.x[p] += action;
        new_state.x[q] += action;
        new_state.x[r] += action;

        new_state.score += new_state.x.iter().filter(|&&v| v == 0).count();
        new_state.turn += 1;
        new_state.history.push(action != 1);
        new_state
    }

    fn print_history(&self) {
        for &b in self.history.iter() {
            if b {
                println!("B");
            } else {
                println!("A");
            }
        }
    }
}

/***********************************************************
* Solution
************************************************************/
fn beam_search(input: &Input) -> State {
    let mut beam: Vec<State> = Vec::new();
    beam.push(State::new());

    for turn in 0..input.T {
        let mut next_beam: Vec<State> = Vec::with_capacity(beam.len() * 2);
        for state in beam.iter() {
            let p = input.P[turn];
            let q = input.Q[turn];
            let r = input.R[turn];

            next_beam.push(state.operate(p, q, r, 1));
            next_beam.push(state.operate(p, q, r, -1));
        }
        // もし候補状態が膨大になったら、得点が高い状態上位 BEAM_WIDTH 件だけを残す
        if next_beam.len() > BEAM_WIDTH {
            next_beam.select_nth_unstable_by_key(BEAM_WIDTH, |s| Reverse(s.score));
            next_beam.truncate(BEAM_WIDTH);
        }
        beam = next_beam;
    }

    // 累積得点が最大の状態を選ぶ
    beam.iter().max_by_key(|s| s.score).unwrap().clone()
}

fn main() {
    let input = Input::read_input();
    let best_state = beam_search(&input);
    best_state.print_history();
}
