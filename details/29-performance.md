# 第 29 章：效能最佳化

> Rust 提供了優秀的預設效能，但了解最佳化技巧可以進一步提升程式效能

---

## 29.1 基準測試

### Criterion

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });

    // 比較多個實作
    let mut group = c.benchmark_group("Compare");
    group.bench_function("recursive", |b| b.iter(|| fibonacci(black_box(20))));
    group.bench_function("iterative", |b| b.iter(|| fibonacci_iter(black_box(20))));
    group.finish();
}

fn fibonacci_iter(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    a
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

---

## 29.2 分析工具

### perf 和 flamegraph

```bash
# 安裝
cargo install flamegraph

# 產生火焰圖
cargo flamegraph

# 使用 perf
perf record -g ./target/release/my_app
perf report
```

### 記憶體分析

```bash
# Valgrind
valgrind --tool=massif ./target/release/my_app
ms_print massif.out.*

# Heaptrack
heaptrack ./target/release/my_app
heaptrack_gui heaptrack.*.gz
```

---

## 29.3 最佳化技巧

### 編譯器最佳化

```toml
# Cargo.toml
[profile.release]
opt-level = 3        # 最大最佳化
lto = true           # Link-Time Optimization
codegen-units = 1    # 更好的最佳化
panic = "abort"      # 減少程式碼大小
strip = true         # 移除符號
```

### 內聯提示

```rust
#[inline]
fn small_function(x: i32) -> i32 {
    x + 1
}

#[inline(always)]
fn must_inline(x: i32) -> i32 {
    x * 2
}

#[inline(never)]
fn never_inline(x: i32) -> i32 {
    x - 1
}
```

### 避免不必要的配置

```rust
// 不好：每次迭代都配置新的 String
fn bad_concat(items: &[&str]) -> String {
    let mut result = String::new();
    for item in items {
        result = result + item;  // 每次都配置新記憶體
    }
    result
}

// 好：原地修改
fn good_concat(items: &[&str]) -> String {
    let total_len: usize = items.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    for item in items {
        result.push_str(item);
    }
    result
}

// 更好：使用 join
fn best_concat(items: &[&str]) -> String {
    items.join("")
}
```

### 選擇正確的資料結構

```rust
use std::collections::{HashMap, BTreeMap, HashSet};

fn main() {
    // HashMap: O(1) 查找，但無序
    let hash_map: HashMap<&str, i32> = HashMap::new();

    // BTreeMap: O(log n) 查找，有序
    let btree_map: BTreeMap<&str, i32> = BTreeMap::new();

    // Vec: 連續記憶體，cache 友好
    let vec: Vec<i32> = Vec::new();

    // 小集合用 Vec 可能比 HashSet 更快
    let small_set: Vec<i32> = vec![1, 2, 3];  // 線性搜尋對小集合更快
}
```

---

## 29.4 常見效能陷阱

```rust
// 1. 不必要的克隆
fn bad(s: &String) {
    let owned = s.clone();  // 不必要的克隆
    println!("{}", owned);
}

fn good(s: &str) {
    println!("{}", s);  // 直接使用引用
}

// 2. 過度使用迭代器 collect
fn bad_filter(v: &[i32]) -> Vec<i32> {
    let filtered: Vec<i32> = v.iter()
        .filter(|&&x| x > 0)
        .cloned()
        .collect();
    filtered.iter().map(|&x| x * 2).collect()  // 兩次配置
}

fn good_filter(v: &[i32]) -> Vec<i32> {
    v.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()  // 一次配置
}

// 3. 字串格式化開銷
fn bad_format(name: &str) -> String {
    format!("{}", name)  // 不必要的格式化
}

fn good_format(name: &str) -> String {
    name.to_string()  // 直接轉換
}
```

---

## 練習題

1. 比較不同排序演算法的效能
2. 最佳化一個字串處理函式
3. 分析並改善記憶體使用

---

## 延伸閱讀

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
