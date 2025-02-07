#![allow(non_snake_case, unused_macros, unused_imports, dead_code, unused_mut)]
use ac_library::*;
use proconio::marker::*;
use proconio::*;
use std::collections::*;
use std::fmt::Debug;
use std::str::FromStr;

/***********************************************************
* Bitwise Calculations
************************************************************/
/// 整数 `num` の `shift` ビット目が1であるかどうかを確認する。
/// ビット位置は0から始まり、0が最下位ビットを表す。
///
/// # 引数
///
/// * `num` - 判定対象の整数。
/// * `shift` - チェックするビットの位置。0が最下位ビット。
///
/// # 戻り値
///
/// 指定されたビット位置にビットが立っている場合は `true`、そうでない場合は `false` 。
fn has_bit(num: i64, shift: u32) -> bool {
    ((num >> shift) & 1) == 1
}

/***********************************************************
* String
************************************************************/
/// 与えられたイテレータの各要素を文字列に変換し、
/// 指定された区切り文字で連結した1つの文字列を返す。
///
/// # 例
/// ```
/// let v = vec![1, 2, 3];
/// let joined = join_with(v, " ");
/// assert_eq!(joined, "1 2 3");
/// ```
fn join_with<I>(iter: I, sep: &str) -> String
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    iter.into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

/***********************************************************
* Number Theory
************************************************************/
/// 指定された整数の素因数分解を行う。
///
/// 与えられた正の整数 `n` を素因数分解し、
/// 各素因数とその指数を `[素因数, 指数]` の形式の配列として `Vec` に格納して返す。
///
/// # 例
///
/// ```
/// let factors = factorization(12);
/// // factors は [[2, 2], [3, 1]] となる
/// ```
fn factorization(n: i64) -> Vec<[i64; 2]> {
    let mut arr = Vec::new();
    let mut temp = n;
    let limit = (n as f64).sqrt().ceil() as i64;

    for i in 2..=limit {
        if temp % i == 0 {
            let mut cnt = 0;
            while temp % i == 0 {
                cnt += 1;
                temp /= i;
            }
            arr.push([i, cnt]);
        }
    }

    if temp != 1 {
        arr.push([temp, 1]);
    }

    if arr.is_empty() {
        arr.push([n, 1]);
    }

    arr
}

/// 指定された整数の全ての正の約数を取得し、昇順に並べたベクターを返す。
///
/// # 引数
///
/// * `n` - 約数を求めたい正の整数。
///
/// # 戻り値
///
/// `n` の全ての正の約数を昇順に並べた `Vec<i64>` を返す。
///
/// # 例
///
/// ```
/// let divs = divisors(36);
/// // 結果は [1, 2, 3, 4, 6, 9, 12, 18, 36] となる
/// ```
fn divisors(n: i64) -> Vec<i64> {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            l1.push(i);
            if i != n / i {
                l2.push(n / i);
            }
        }
        i += 1;
    }
    l2.reverse();
    l1.extend(l2);
    l1
}

/***********************************************************
* Encoding
************************************************************/
/// 座標圧縮
///
/// # 引数
/// - `a`: 座標圧縮を行う整数のベクター
///
/// # 戻り値
/// 元のベクターの各要素を一意なランク（1始まり）に置換した新たなベクターを返す。
///
/// # 例
/// ```
/// let v = vec![40, 10, 20, 20, 30];
/// let compressed = compress(&v);
/// assert_eq!(compressed, vec![4, 1, 2, 2, 3]);
/// ```
fn compress(a: &[i64]) -> Vec<i64> {
    let mut b = a.to_vec();
    b.sort();
    b.dedup();

    let mut rank: HashMap<i64, i64> = HashMap::new();
    for (i, &x) in b.iter().enumerate() {
        rank.insert(x, i as i64 + 1);
    }

    a.iter().map(|&x| rank[&x]).collect()
}

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
        N: f64,
    }

    let mut left = 0f64;
    let mut right = N as f64;
    fn cal(x: f64) -> f64 {
        x * x * x + x
    }
    for _ in 0..1000 {
        let mid = (left + right) / 2.0;
        if cal(mid) < N {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
