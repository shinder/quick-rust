# 第 1 章：認識 Rust

> Rust 是一門專注於安全、並行與效能的系統程式語言，讓你能夠寫出既快速又可靠的軟體。

---

## 1.1 Rust 是什麼？

### Rust 的歷史與發展

Rust 的故事始於 2006 年，當時 Mozilla 工程師 Graydon Hoare 開始這個個人專案。2009 年，Mozilla 正式贊助 Rust 的開發，希望用它來建構下一代的瀏覽器引擎。

**重要里程碑：**

| 年份 | 事件 |
|------|------|
| 2006 | Graydon Hoare 開始開發 Rust |
| 2009 | Mozilla 開始贊助 |
| 2010 | Rust 首次公開發表 |
| 2015 | Rust 1.0 正式發布 |
| 2021 | Rust Foundation 成立（AWS、Google、Microsoft、Mozilla、Huawei 為創始成員） |
| 2022 | Linux 核心開始支援 Rust |
| 2023 | Windows 核心開始整合 Rust |

Rust 連續多年在 Stack Overflow 開發者調查中被評選為「最受喜愛的程式語言」，這反映了開發者對其設計理念的認同。

### 設計目標

Rust 的設計有三大核心目標：

#### 1. 安全（Safety）

Rust 在編譯時期就能捕捉到大部分的記憶體錯誤，包括：

- **空指標解引用**（Null pointer dereference）
- **緩衝區溢位**（Buffer overflow）
- **懸垂指標**（Dangling pointer）
- **資料競爭**（Data race）
- **使用已釋放的記憶體**（Use after free）

這些在 C/C++ 中常見的錯誤，在 Rust 中幾乎不可能發生（除非使用 `unsafe` 區塊）。

```rust
// 這段程式碼無法編譯 - Rust 會在編譯時期阻止錯誤
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // 錯誤：x 的生命週期不夠長
    }
    // println!("{}", r);  // x 已經被丟棄，r 是懸垂引用
}
```

#### 2. 並行（Concurrency）

Rust 的所有權系統讓編譯器能在編譯時期檢測資料競爭。這意味著：

- **無懼並行**（Fearless concurrency）
- 編譯通過的程式碼不會有資料競爭
- 多執行緒程式設計更加安全

```rust
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    // 這無法編譯 - 不能同時有多個可變引用
    // let handle = thread::spawn(|| {
    //     data.push(4);  // 錯誤：無法在另一個執行緒中借用
    // });

    // 正確做法：使用 move 轉移所有權
    let handle = thread::spawn(move || {
        data.push(4);
        data
    });

    let result = handle.join().unwrap();
    println!("{:?}", result);  // [1, 2, 3, 4]
}
```

#### 3. 高效能（Performance）

Rust 的效能可媲美 C/C++：

- **無垃圾回收**：沒有 GC 暫停
- **零成本抽象**：高階抽象不犧牲效能
- **精細的記憶體控制**：可以精確控制記憶體配置
- **編譯時期最佳化**：LLVM 後端提供強大的最佳化

```rust
// 這段高階程式碼編譯後的效能等同於手寫的低階迴圈
let sum: i32 = (1..=100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .sum();
```

### 適用場景

Rust 特別適合以下領域：

#### 系統程式設計

- 作業系統核心（Linux、Windows 已開始採用）
- 裝置驅動程式
- 嵌入式系統
- 虛擬機器、容器執行時（如 Firecracker）

#### 網路服務

- 高效能 Web 服務（Cloudflare、Discord 使用 Rust）
- 代理伺服器
- 資料庫系統

#### 命令列工具

- ripgrep（比 grep 快得多的搜尋工具）
- bat（帶語法高亮的 cat）
- exa（現代化的 ls）
- fd（更友善的 find）

#### WebAssembly

- 前端效能關鍵程式碼
- 瀏覽器外掛
- Serverless 函式

#### 區塊鏈與加密

- Solana、Polkadot 等區塊鏈
- 加密貨幣錢包
- 密碼學函式庫

---

## 1.2 Rust 與其他語言的比較

### Rust vs C/C++

| 特性 | C/C++ | Rust |
|------|-------|------|
| 記憶體安全 | 手動管理，容易出錯 | 編譯時期保證 |
| 空指標 | 常見錯誤來源 | 無空指標，使用 Option |
| 資料競爭 | 執行時可能發生 | 編譯時期防止 |
| 未定義行為 | 存在 | 幾乎不存在（除 unsafe） |
| 建置系統 | Make、CMake 等 | Cargo（統一標準） |
| 套件管理 | 無標準方案 | crates.io |
| 學習曲線 | 陡峭（手動管理） | 陡峭（借用檢查器） |
| 生態系成熟度 | 非常成熟 | 快速成長中 |

**何時選擇 Rust 而非 C/C++：**

- 重視安全性的新專案
- 需要現代化的套件管理
- 希望減少記憶體相關的 bug
- 團隊有意願學習新語言

**何時選擇 C/C++ 而非 Rust：**

- 維護現有的 C/C++ 程式碼庫
- 需要最廣泛的平台支援
- 團隊已有 C/C++ 專業知識
- 需要使用特定的 C/C++ 函式庫

### Rust vs Go

| 特性 | Go | Rust |
|------|-----|------|
| 記憶體管理 | 垃圾回收（GC） | 所有權系統（無 GC） |
| 效能 | 優秀 | 卓越（接近 C） |
| 型別系統 | 簡單（無泛型*） | 豐富（泛型、trait） |
| 錯誤處理 | 多回傳值 | Result 類型 |
| 並行模型 | Goroutine + Channel | 執行緒 + 訊息傳遞 |
| 編譯速度 | 非常快 | 較慢 |
| 學習曲線 | 平緩 | 陡峭 |
| 空值 | nil（可能 panic） | Option（編譯時檢查） |

*註：Go 1.18 已加入泛型支援

**何時選擇 Rust 而非 Go：**

- 需要最高效能
- 不能接受 GC 暫停
- 需要精細的記憶體控制
- 系統程式設計、嵌入式開發

**何時選擇 Go 而非 Rust：**

- 需要快速開發
- 團隊學習成本考量
- 微服務、API 開發
- DevOps 工具開發

### Rust vs JavaScript/TypeScript

| 特性 | JavaScript/TypeScript | Rust |
|------|----------------------|------|
| 執行環境 | 直譯（V8 等引擎） | 編譯為機器碼 |
| 型別系統 | 動態（JS）/ 靜態（TS） | 靜態強型別 |
| 效能 | 中等（JIT 最佳化） | 卓越 |
| 記憶體管理 | 垃圾回收 | 所有權系統 |
| 空值處理 | null/undefined | Option |
| 錯誤處理 | try/catch | Result |
| 並行 | 單執行緒 + Event Loop | 多執行緒 |

```javascript
// JavaScript - 可能在執行時期出錯
function processUser(user) {
    return user.name.toUpperCase();  // user 或 name 可能是 null
}
```

```rust
// Rust - 編譯時期就必須處理所有情況
fn process_user(user: Option<User>) -> Option<String> {
    user.map(|u| u.name.to_uppercase())
}
```

**何時選擇 Rust 而非 JavaScript：**

- 效能關鍵的後端服務
- WebAssembly 模組
- 命令列工具
- 需要多執行緒並行

**何時選擇 JavaScript 而非 Rust：**

- 前端網頁開發
- 快速原型開發
- 團隊已有 JS 專業知識
- 需要龐大的 npm 生態系

### Rust vs Python

| 特性 | Python | Rust |
|------|--------|------|
| 執行方式 | 直譯 | 編譯 |
| 效能 | 較慢 | 極快（10-100倍） |
| 型別系統 | 動態 | 靜態強型別 |
| 記憶體管理 | 垃圾回收 | 所有權系統 |
| 學習曲線 | 平緩 | 陡峭 |
| 開發速度 | 快 | 較慢 |
| 部署 | 需要直譯器 | 單一執行檔 |

**Rust + Python 的完美組合：**

許多專案使用 Python 作為高階介面，Rust 作為效能關鍵的底層：

- **PyO3**：讓你用 Rust 寫 Python 擴充
- **Polars**：Rust 寫的 DataFrame 函式庫，比 Pandas 快很多
- **Ruff**：Rust 寫的 Python linter，速度驚人

```python
# 使用 Rust 擴充的 Python 程式碼
import polars as pl  # Rust 實作的 DataFrame

df = pl.read_csv("data.csv")
result = df.filter(pl.col("age") > 30).select(["name", "age"])
```

---

## 1.3 Rust 的核心理念

### 零成本抽象（Zero-cost Abstractions）

「你不用的東西，不需要付出代價。更重要的是，你用的東西，不可能寫得比這更好。」
— Bjarne Stroustrup

Rust 繼承了這個理念。高階的抽象（如迭代器、泛型）在編譯後不會產生額外開銷：

```rust
// 高階寫法
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();

// 編譯後的效能等同於這個低階寫法
let mut sum = 0;
for x in &[1, 2, 3, 4, 5] {
    if *x % 2 == 0 {
        sum += x * x;
    }
}
```

編譯器會自動將高階的迭代器鏈內聯展開，產生與手寫迴圈相同的機器碼。

### 無垃圾回收（No Garbage Collection）

傳統的記憶體管理方式：

1. **手動管理**（C/C++）：程式設計師負責 malloc/free，容易出錯
2. **垃圾回收**（Java、Go、JS）：自動回收，但有 GC 暫停

Rust 選擇第三條路：

3. **所有權系統**：編譯時期決定何時釋放記憶體

```rust
fn main() {
    let s1 = String::from("hello");  // s1 擁有這個字串

    let s2 = s1;  // 所有權移動到 s2，s1 不再有效

    // println!("{}", s1);  // 編譯錯誤！s1 已經無效

    println!("{}", s2);  // OK

}  // s2 離開作用域，記憶體自動釋放
```

**優點：**

- 沒有 GC 暫停，效能可預測
- 沒有 GC 執行時開銷
- 記憶體使用更少
- 適合即時系統、嵌入式系統

### 編譯時期保證記憶體安全

Rust 的借用檢查器（Borrow Checker）在編譯時期執行嚴格的檢查：

#### 規則 1：一次只能有一個可變引用

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s;  // 錯誤！不能同時有兩個可變引用

r1.push_str(" world");
println!("{}", r1);
```

#### 規則 2：不能同時有可變和不可變引用

```rust
let mut s = String::from("hello");

let r1 = &s;     // 不可變引用
let r2 = &s;     // 可以有多個不可變引用
// let r3 = &mut s;  // 錯誤！不能同時有可變和不可變引用

println!("{}, {}", r1, r2);
```

#### 規則 3：引用必須有效

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // 錯誤！x 的生命週期不夠長
    }
    // println!("{}", r);  // x 已經被丟棄
}
```

這些規則確保：

- **無資料競爭**：不可能有兩個執行緒同時修改同一資料
- **無懸垂指標**：不可能存取已釋放的記憶體
- **無緩衝區溢位**：陣列存取會進行邊界檢查

### 「如果能編譯，就能正確執行」

雖然這個說法有些誇張，但 Rust 的嚴格檢查確實大幅減少了執行時期錯誤：

```rust
// Rust 強制你處理所有可能的情況
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    // 必須處理 None 的情況
    match divide(10.0, 0.0) {
        Some(result) => println!("結果: {}", result),
        None => println!("不能除以零"),
    }
}
```

---

## 本章重點回顧

1. **Rust 是什麼**
   - 專注於安全、並行、效能的系統程式語言
   - 由 Mozilla 發起，現由 Rust Foundation 管理
   - 適用於系統程式、網路服務、CLI、WebAssembly

2. **與其他語言比較**
   - vs C/C++：更安全，有現代化工具鏈
   - vs Go：無 GC，效能更高，型別系統更豐富
   - vs JavaScript：編譯語言，效能更好，適合不同場景

3. **核心理念**
   - 零成本抽象：高階抽象不犧牲效能
   - 無垃圾回收：所有權系統管理記憶體
   - 編譯時期安全：借用檢查器防止記憶體錯誤

---

## 練習與思考

1. **思考題**：你目前的專案中，哪些部分可能適合用 Rust 重寫？為什麼？

2. **調查作業**：找出三個使用 Rust 開發的知名開源專案，了解它們為什麼選擇 Rust。

3. **比較練習**：用你熟悉的語言寫一個簡單的程式，思考如果用 Rust 寫，會有什麼不同？

---

## 延伸閱讀

- [Rust 官方網站](https://www.rust-lang.org/)
- [Rust 程式語言（官方書籍）](https://doc.rust-lang.org/book/)
- [Rust Foundation](https://foundation.rust-lang.org/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)

---

[下一章：環境建置 →](./02-environment.md)
