# 第 27 章：巨集系統

> Rust 的巨集系統提供了強大的元程式設計能力，從簡單的程式碼生成到複雜的 DSL

---

## 27.1 宣告式巨集

### macro_rules! 語法

```rust
// 簡單巨集
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 帶參數
macro_rules! print_msg {
    ($msg:expr) => {
        println!("Message: {}", $msg);
    };
}

// 多種匹配
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
}

fn main() {
    say_hello!();
    print_msg!("Hello, Macro!");
    println!("{}", calculate!(add 2, 3));
    println!("{}", calculate!(mul 4, 5));
}
```

### 重複模式

```rust
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$($x.to_string()),*]
    };
}

macro_rules! hash_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

fn main() {
    let strings = vec_of_strings!["a", "b", "c"];
    println!("{:?}", strings);

    let map = hash_map! {
        "a" => 1,
        "b" => 2,
        "c" => 3,
    };
    println!("{:?}", map);
}
```

### 衛生性

```rust
macro_rules! using_a {
    ($e:expr) => {{
        let a = 42;  // 這個 a 不會與外部衝突
        $e
    }};
}

fn main() {
    let a = 13;
    // 使用的是巨集內部的 a
    let result = using_a!(a);
    println!("a in macro: {}", result); // 42
    println!("a outside: {}", a);       // 13
}
```

---

## 27.2 程序式巨集

### derive 巨集

```toml
# Cargo.toml (巨集 crate)
[lib]
proc-macro = true

[dependencies]
quote = "1"
syn = "2"
proc-macro2 = "1"
```

```rust
// my_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, I'm {}!", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
```

```rust
// 使用
use my_derive::HelloMacro;

trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

### 屬性巨集

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let path = attr.to_string();

    let expanded = quote! {
        #input

        fn register_#name() {
            println!("Registering route: {}", #path);
        }
    };

    TokenStream::from(expanded)
}

// 使用
#[route("/api/users")]
fn get_users() {}
```

---

## 27.3 常用巨集模式

### 建構器模式

```rust
macro_rules! builder {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        pub struct $name {
            $($field: Option<$type>),*
        }

        impl $name {
            pub fn new() -> Self {
                $name {
                    $($field: None),*
                }
            }

            $(
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
        }
    };
}

builder!(RequestBuilder {
    url: String,
    method: String,
    body: String,
});

fn main() {
    let req = RequestBuilder::new()
        .url("https://example.com".into())
        .method("GET".into());
}
```

---

## 練習題

1. 建立一個 `debug_print!` 巨集
2. 實作 `#[derive(Default)]` 類似的功能
3. 設計一個簡單的 DSL

---

## 延伸閱讀

- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/)
- [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
