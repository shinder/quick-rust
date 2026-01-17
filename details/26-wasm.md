# 第 26 章：WebAssembly

> Rust 是開發 WebAssembly 的最佳語言之一，提供優秀的效能和小體積

---

## 26.1 WASM 基礎

### 設置環境

```bash
# 安裝 wasm-pack
cargo install wasm-pack

# 建立專案
cargo new --lib wasm-hello
cd wasm-hello
```

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
# 編譯
wasm-pack build --target web
```

---

## 26.2 與 JavaScript 互動

```rust
use wasm_bindgen::prelude::*;

// 呼叫 JavaScript 函式
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

// 匯出結構體
#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator { value: 0.0 }
    }

    pub fn add(&mut self, n: f64) {
        self.value += n;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[wasm_bindgen]
pub fn run() {
    log("Hello from Rust!");
    alert("WASM is running!");
}
```

### HTML 整合

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>WASM Demo</title>
</head>
<body>
    <script type="module">
        import init, { greet, Calculator } from './pkg/wasm_hello.js';

        async function run() {
            await init();

            console.log(greet("World"));

            const calc = new Calculator();
            calc.add(5);
            calc.add(3);
            console.log("Result:", calc.get_value());
        }

        run();
    </script>
</body>
</html>
```

---

## 26.3 WASM 應用

### web-sys（DOM 操作）

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlElement", "Window"] }
```

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, window};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = window().expect("no window");
    let document = window.document().expect("no document");

    let body = document.body().expect("no body");

    let div = document.create_element("div")?;
    div.set_inner_html("<h1>Hello from Rust!</h1>");
    body.append_child(&div)?;

    Ok(())
}
```

### Yew（前端框架）

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
```

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <h1>{ "Counter" }</h1>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

---

## 練習題

1. 建立一個 WASM 計算機
2. 使用 Yew 建立 TODO 應用
3. 實作圖片處理 WASM 模組

---

## 延伸閱讀

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Yew Documentation](https://yew.rs/)
