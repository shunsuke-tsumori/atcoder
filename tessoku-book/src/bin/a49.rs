#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::input;
use std::cmp::Reverse;

const END_TURN: usize = 100;
const BEAM_WIDTH: usize = 60_000;

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
#[derive(Debug, Clone, Copy)]
struct State {
    x: [i8; 20],
    turn: usize,
    history: [bool; END_TURN],  // 各ターンの操作記録。false: A, true: B
    score: usize,
}

impl State {
    #[inline(always)]
    fn new() -> Self {
        Self {
            x: [0; 20],
            turn: 0,
            history: [false; END_TURN],
            score: 0,
        }
    }

    // action が +1 なら操作 A、-1 なら操作 B とする。
    #[inline(always)]
    fn operate(&self, p: usize, q: usize, r: usize, action: i8) -> Self {
        let mut new_x = self.x;
        new_x[p] += action;
        new_x[q] += action;
        new_x[r] += action;
        // 現在の状態で x[j]==0 となっている要素数を加点
        let mut add_score = 0;
        for &v in new_x.iter() {
            if v == 0 {
                add_score += 1;
            }
        }
        // history をコピーして、現在の turn 番目に今回の操作を記録する。
        let mut new_history = self.history;
        new_history[self.turn] = action != 1; // action==1なら false (A), それ以外なら true (B)
        Self {
            x: new_x,
            turn: self.turn + 1,
            history: new_history,
            score: self.score + add_score,
        }
    }

    fn print_history(&self) {
        for i in 0..self.turn {
            if self.history[i] {
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
    let mut beam: Vec<State> = Vec::with_capacity(BEAM_WIDTH);
    beam.push(State::new());
    let mut next_beam: Vec<State> = Vec::with_capacity(beam.len() * 2);

    for turn in 0..input.T {
        next_beam.clear();
        for state in beam.iter() {
            let p = input.P[turn];
            let q = input.Q[turn];
            let r = input.R[turn];
            next_beam.push(state.operate(p, q, r, 1));
            next_beam.push(state.operate(p, q, r, -1));
        }
        // 状態数がビーム幅を超えたら、部分ソートで上位 BEAM_WIDTH 件のみを残す
        if next_beam.len() > BEAM_WIDTH {
            next_beam.select_nth_unstable_by_key(BEAM_WIDTH, |s| Reverse(s.score));
            next_beam.truncate(BEAM_WIDTH);
        }
        std::mem::swap(&mut beam, &mut next_beam);
    }
    // beam 内で最大の累積得点を持つ状態を返す
    *beam.iter().max_by_key(|s| s.score).unwrap()
}

fn main() {
    let input = Input::read_input();
    let best_state = beam_search(&input);
    best_state.print_history();
}
