#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use proconio::marker::*;
use proconio::*;
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
pub fn run_length_encode<T>(data: &[T]) -> Vec<(T, usize)>
where
    T: Eq + Clone,
{
    let mut result = Vec::new();
    if data.is_empty() {
        return result;
    }

    let mut current_value = data[0].clone();
    let mut current_count = 1;

    for i in 1..data.len() {
        if data[i] == current_value {
            current_count += 1;
        } else {
            result.push((current_value, current_count));
            current_value = data[i].clone();
            current_count = 1;
        }
    }

    result.push((current_value, current_count));
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
    let mut result = Vec::new();
    for (value, count) in encoded {
        for _ in 0..*count {
            result.push(value.clone());
        }
    }
    result
}

fn main() {
    input! {
        N: usize,
        S: Chars
    }
}
