# atcoder

```bash
cargo compete new abc200
```

## 参考

https://zenn.dev/ne0/articles/39ddc9fb2d4123
https://qiita.com/butzsuppin/items/ab9d86177a1c46b108d5
https://github.com/rust-lang-ja/ac-library-rs/tree/master/examples

## Rustメモ

### 数字（n進数）

```rust
fn main() {
    let num = 255;

    // プレフィックスなし
    println!("2進数: {:b}", num);  // 11111111
    println!("2進数: {:010b}", num);  // 0011111111
    println!("8進数: {:o}", num);  // 377
    println!("16進数: {:x}", num); // ff
    println!("16進数 (大文字): {:X}", num); // FF

    // プレフィックス付き
    println!("2進数 (プレフィックス付き): {:#b}", num);  // 0b11111111
    println!("8進数 (プレフィックス付き): {:#o}", num);  // 0o377
    println!("16進数 (プレフィックス付き): {:#x}", num); // 0xff
}
```

```rust
fn main() {
    // 2進数の文字列を10進数の u32 に変換 (基数: 2)
    let bin_str = "1010"; // 10進数の10に相当
    let num_from_bin = u32::from_str_radix(bin_str, 2)
        .expect("Invalid binary string");
    println!("2進数 {} -> {}", bin_str, num_from_bin);

    // 8進数の文字列を10進数の u32 に変換 (基数: 8)
    let oct_str = "12";   // 10進数の10に相当
    let num_from_oct = u32::from_str_radix(oct_str, 8)
        .expect("Invalid octal string");
    println!("8進数 {} -> {}", oct_str, num_from_oct);

    // 16進数の文字列を10進数の u32 に変換 (基数: 16)
    let hex_str = "A";    // 10進数の10に相当
    let num_from_hex = u32::from_str_radix(hex_str, 16)
        .expect("Invalid hexadecimal string");
    println!("16進数 {} -> {}", hex_str, num_from_hex);
}
```

### 言語的注意点

- usizeのアンダーフローに注意（特に二分探索）
