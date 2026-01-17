# 第 28 章：Unsafe Rust

> Unsafe Rust 讓你能夠繞過編譯器的安全檢查，但需要你自己保證程式碼的正確性

---

## 28.1 Unsafe 基礎

### 為什麼需要 unsafe

```rust
fn main() {
    // 有些操作編譯器無法驗證安全性
    // 需要程式設計師自己保證

    // 1. 解引用裸指標
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 在 safe Rust 中無法解引用
    // 必須使用 unsafe
    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
        println!("r2: {}", *r2);
    }
}
```

### unsafe 區塊

```rust
fn main() {
    let mut data = [1, 2, 3, 4, 5];

    unsafe {
        // 解引用裸指標
        let ptr = data.as_mut_ptr();
        *ptr.add(2) = 100;
    }

    println!("{:?}", data); // [1, 2, 100, 4, 5]
}
```

### unsafe 函式

```rust
// 宣告 unsafe 函式
unsafe fn dangerous() {
    // 這個函式的呼叫者必須確保安全
    println!("這是危險的程式碼");
}

fn main() {
    // 必須在 unsafe 區塊中呼叫
    unsafe {
        dangerous();
    }
}

// 將 unsafe 封裝在 safe 介面中
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

---

## 28.2 Unsafe 能力

### 解引用裸指標

```rust
fn main() {
    let mut num = 5;

    // 建立裸指標
    let r1: *const i32 = &num;
    let r2: *mut i32 = &mut num;

    // 可以在 safe code 中建立裸指標，但不能解引用
    println!("r1 addr: {:p}", r1);

    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
    }

    // 從任意地址建立指標（非常危險！）
    let address = 0x012345usize;
    let _r = address as *const i32;
    // unsafe { println!("{}", *_r); } // 未定義行為！
}
```

### 存取可變靜態變數

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### 實作 unsafe trait

```rust
unsafe trait Foo {
    fn do_something(&self);
}

struct MyType;

// 實作者保證安全性
unsafe impl Foo for MyType {
    fn do_something(&self) {
        println!("Doing something");
    }
}
```

---

## 28.3 FFI（外部函式介面）

### 呼叫 C 函式

```rust
// 連結 C 標準函式庫
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
        println!("sqrt(4.0) = {}", sqrt(4.0));
    }
}
```

### 被 C 呼叫

```rust
// 讓 Rust 函式可以被 C 呼叫
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}
```

### bindgen

```toml
# Cargo.toml
[build-dependencies]
bindgen = "0.69"
```

```rust
// build.rs
fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
```

---

## 28.4 安全抽象

```rust
// 封裝 unsafe 程式碼，提供 safe API
pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    // Safe 介面
    pub fn push(&mut self, value: T) {
        // unsafe 實作細節
        // ...
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.ptr.add(index)) }
        } else {
            None
        }
    }
}
```

---

## 練習題

1. 實作一個簡單的記憶體配置器
2. 封裝一個 C 函式庫
3. 實作自訂的 `Vec`

---

## 延伸閱讀

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
