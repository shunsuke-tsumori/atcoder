#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use std::fmt::Debug;
use std::str::FromStr;

/***********************************************************
* I/O
************************************************************/
/// 与えられた行数 h, 列数 w に従い、
/// 標準入力から h 行分の [T; w] を読み込んで Vec<Vec<T>> を返す。
/// `is_1_indexed` が true の場合は、1-indexed として扱えるように先頭行・先頭列にダミーを挿入して返す。
pub fn input_grid<T>(h: usize, w: usize, is_1_indexed: bool) -> Vec<Vec<T>>
where
    T: FromStr + Default,
    <T as FromStr>::Err: Debug,
{
    if !is_1_indexed {
        // 0-indexed
        let mut grid = Vec::with_capacity(h);
        for _ in 0..h {
            input! {
                row: [T; w],
            }
            grid.push(row);
        }
        grid
    } else {
        // 1-indexed
        // 0 行目と 0 列目をダミーとして確保し、実際の入力は 1..=h, 1..=w のインデックスに格納する。
        let mut grid = Vec::with_capacity(h + 1);

        // 0 行目はダミーの空ベクタにしておく
        grid.push(Vec::new());

        for _ in 0..h {
            input! {
                row: [T; w],
            }
            let mut new_row = Vec::with_capacity(w + 1);
            new_row.push(T::default()); // 0 列目のダミー
            new_row.extend(row);
            grid.push(new_row);
        }
        grid
    }
}

/***********************************************************
* Encoding
************************************************************/
/// ランレングス圧縮
///
/// # 使用例
///     run_length_encode("aaabbccccaa".chars())
///
/// # 引数
///
/// - `data`: 圧縮対象のデータスライス。要素が `Eq` と `Clone` を実装している必要がある。
///
/// # 戻り値
///
/// `(T, usize)` のベクタ。`T` は各区間の要素、`usize` は連続して現れた回数。
///
/// # 例: vec![('a', 3), ('b', 2), ('c', 4), ('a', 2)]
///
pub fn run_length_encode<I, T>(data: I) -> Vec<(T, usize)>
where
    I: IntoIterator<Item = T>,
    T: Eq + Clone,
{
    let mut iter = data.into_iter();
    let first = match iter.next() {
        Some(x) => x,
        None => return Vec::new(),
    };

    let mut current = first;
    let mut count = 1;
    let mut result = Vec::new();

    for item in iter {
        if item == current {
            count += 1;
        } else {
            result.push((current, count));
            current = item;
            count = 1;
        }
    }
    result.push((current, count));
    result
}

/// ランレングス圧縮のデコード
///
/// # 引数
///
/// - `encoded`: ランレングス圧縮された `(T, usize)` のスライス
///
/// # 戻り値
///
/// 元のデータ列を格納した `Vec<T>`
///
/// # 例
///
/// ```
/// let encoded = vec![('a', 3), ('b', 2), ('c', 4), ('a', 2)];
/// let decoded = run_length_decode(&encoded);
/// // => ['a', 'a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'a', 'a']
/// println!(decoded.iter().collect::<String>())
/// // => "aaabbccccaa"
/// ```
pub fn run_length_decode<T>(encoded: &[(T, usize)]) -> Vec<T>
where
    T: Clone,
{
    encoded
        .iter()
        .flat_map(|(value, count)| std::iter::repeat(value.clone()).take(*count))
        .collect()
}

#[fastout] // インタラクティブでは外す
fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [[usize; 2]; N],
    }
    let mut cm = vec![0; D + 3];
    for i in 0..N {
        cm[LR[i][0]] += 1;
        cm[LR[i][1] + 1] -= 1
    }
    for i in 1..D + 1 {
        cm[i] += cm[i - 1];
        println!("{}", cm[i]);
    }
}
