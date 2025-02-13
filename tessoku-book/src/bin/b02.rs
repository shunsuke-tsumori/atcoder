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

fn main() {
    input! {
        A: i64,
        B: i64,
    }
    let mut divs = divisors(100);
    let divs: HashSet<i64> = divs.into_iter().collect();
    let mut ans = 0;
    for i in A..=B {
        if divs.contains(&i) {
            ans += 1
        }
    }
    let ans = if ans > 0 { "Yes" } else { "No" };
    println!("{}", ans);
}
