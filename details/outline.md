# Rust 完整教學大綱

> 從零基礎到進階應用的完整學習路徑

---

## 第一部分：基礎入門

### 第 1 章：認識 Rust

- 1.1 Rust 是什麼？
  - Rust 的歷史與發展（Mozilla → Rust Foundation）
  - 設計目標：安全、並行、高效能
  - 適用場景：系統程式、WebAssembly、CLI、嵌入式
- 1.2 Rust 與其他語言的比較
  - vs C/C++：記憶體安全，無 undefined behavior
  - vs Go：無 GC、更強的型別系統
  - vs JavaScript/Python：編譯語言、靜態型別
- 1.3 Rust 的核心理念
  - 零成本抽象（Zero-cost abstractions）
  - 無垃圾回收（No GC）
  - 編譯時期保證記憶體安全

### 第 2 章：環境建置

- 2.1 安裝 Rust
  - rustup 安裝與管理
  - 多版本管理（stable、beta、nightly）
  - 跨平台安裝（macOS、Linux、Windows）
- 2.2 開發工具設定
  - VS Code + rust-analyzer
  - IntelliJ IDEA + Rust 插件
  - Vim/Neovim 設定
  - 偵錯工具：CodeLLDB、lldb、gdb
- 2.3 Cargo 套件管理工具
  - 專案建立：`cargo new`、`cargo init`
  - 編譯與執行：`cargo build`、`cargo run`
  - 檢查與測試：`cargo check`、`cargo test`
  - 程式碼品質：`cargo fmt`、`cargo clippy`
- 2.4 Hello World 詳解
  - `fn main()` 進入點
  - `println!` 巨集
  - 編譯過程：原始碼 → MIR → LLVM IR → 機器碼

### 第 3 章：基本語法

- 3.1 變數與可變性
  - `let` 不可變綁定
  - `let mut` 可變綁定
  - `const` 編譯時期常數
  - `static` 靜態變數
  - 變數遮蔽（Shadowing）
- 3.2 基本資料型別
  - 純量型別
    - 整數：i8、i16、i32、i64、i128、isize
    - 無號整數：u8、u16、u32、u64、u128、usize
    - 浮點數：f32、f64
    - 布林：bool
    - 字元：char（Unicode）
  - 複合型別
    - 元組（Tuple）
    - 陣列（Array）
    - 切片（Slice）
- 3.3 型別推導與標註
  - 自動型別推導
  - 明確型別標註
  - 型別別名（type alias）
- 3.4 運算子
  - 算術運算子
  - 比較運算子
  - 邏輯運算子
  - 位元運算子
  - 複合指派運算子

### 第 4 章：控制流程

- 4.1 條件判斷
  - if/else 表達式
  - if let 語法糖
  - 條件作為表達式（回傳值）
- 4.2 迴圈
  - loop 無限迴圈
  - while 條件迴圈
  - while let 迴圈
  - for 迭代迴圈
  - 迴圈標籤與 break/continue
  - loop 回傳值
- 4.3 模式匹配
  - match 表達式
  - 匹配守衛（Match guards）
  - @ 綁定
  - 解構模式
  - 萬用模式 `_`

### 第 5 章：函式與閉包

- 5.1 函式定義
  - 函式簽名
  - 參數與回傳值
  - 表達式 vs 陳述式
  - 提前回傳（early return）
- 5.2 閉包（Closures）
  - 閉包語法
  - 環境捕獲
    - Fn：不可變借用
    - FnMut：可變借用
    - FnOnce：取得所有權
  - move 閉包
  - 閉包作為參數和回傳值
- 5.3 高階函式
  - 函式指標
  - 接受函式作為參數
  - 回傳函式

---

## 第二部分：核心概念

### 第 6 章：所有權系統

- 6.1 所有權基礎
  - 所有權規則
  - 作用域與丟棄（Drop）
  - 堆疊（Stack）vs 堆積（Heap）
- 6.2 移動語義（Move Semantics）
  - 移動與複製
  - Copy trait
  - Clone trait
- 6.3 借用與引用
  - 不可變借用 `&T`
  - 可變借用 `&mut T`
  - 借用規則
  - 懸垂引用（Dangling References）
- 6.4 切片（Slices）
  - 字串切片 `&str`
  - 陣列切片 `&[T]`
  - 切片作為函式參數

### 第 7 章：生命週期

- 7.1 生命週期基礎
  - 為什麼需要生命週期
  - 生命週期標註語法 `'a`
  - 編譯器的生命週期省略規則
- 7.2 函式中的生命週期
  - 輸入生命週期
  - 輸出生命週期
  - 多個生命週期參數
- 7.3 結構體中的生命週期
  - 帶引用的結構體
  - 生命週期與泛型
- 7.4 進階生命週期
  - 靜態生命週期 `'static`
  - 生命週期子型別
  - 生命週期約束

### 第 8 章：結構體

- 8.1 定義結構體
  - 具名欄位結構體
  - 元組結構體
  - 單元結構體（Unit Struct）
- 8.2 實例化與存取
  - 建立實例
  - 欄位簡寫語法
  - 結構體更新語法 `..`
- 8.3 方法與關聯函式
  - impl 區塊
  - self、&self、&mut self
  - 關聯函式（無 self）
  - 多個 impl 區塊
- 8.4 結構體的記憶體佈局
  - 欄位對齊
  - `#[repr(C)]` 與 FFI

### 第 9 章：列舉與模式匹配

- 9.1 列舉定義
  - 簡單列舉
  - 帶資料的變體
  - 類 C 列舉
- 9.2 Option 類型
  - Some 與 None
  - 常用方法：unwrap、expect、map、and_then
  - 空值安全
- 9.3 Result 類型
  - Ok 與 Err
  - 錯誤傳遞 `?` 運算子
  - 常用方法：map、map_err、unwrap_or
- 9.4 模式匹配進階
  - 解構列舉
  - 巢狀模式
  - 範圍模式
  - 參考模式

### 第 10 章：錯誤處理

- 10.1 可恢復錯誤 vs 不可恢復錯誤
  - panic! 與程式終止
  - Result 與錯誤傳遞
- 10.2 Result 深入
  - 錯誤傳遞運算子 `?`
  - 轉換錯誤類型
  - 組合多種錯誤
- 10.3 自訂錯誤類型
  - 實作 std::error::Error
  - 實作 Display 與 Debug
  - 錯誤鏈（Error chaining）
- 10.4 錯誤處理工具箱
  - thiserror crate
  - anyhow crate
  - 何時使用哪個

---

## 第三部分：進階特性

### 第 11 章：泛型

- 11.1 泛型基礎
  - 泛型函式
  - 泛型結構體
  - 泛型列舉
  - 泛型方法
- 11.2 泛型約束
  - Trait bounds
  - where 子句
  - 多重約束
- 11.3 單態化（Monomorphization）
  - 零成本抽象
  - 編譯時期展開
  - 程式碼膨脹問題

### 第 12 章：Trait 系統

- 12.1 Trait 基礎
  - 定義 trait
  - 實作 trait
  - 預設實作
- 12.2 常用標準 Trait
  - Debug、Display
  - Clone、Copy
  - PartialEq、Eq
  - PartialOrd、Ord
  - Default
  - From、Into
  - AsRef、AsMut
- 12.3 Trait 作為參數
  - impl Trait 語法
  - Trait bounds
  - 動態分派 vs 靜態分派
- 12.4 Trait 物件
  - dyn Trait
  - 物件安全（Object safety）
  - 效能考量
- 12.5 進階 Trait
  - 關聯類型
  - 預設泛型參數
  - 運算子重載
  - 完全限定語法
  - Supertraits
  - Newtype 模式

### 第 13 章：集合型別

- 13.1 Vec<T>
  - 建立與操作
  - 容量與長度
  - 迭代與存取
  - 記憶體佈局
- 13.2 String
  - String vs &str
  - UTF-8 編碼
  - 字串操作
  - 字串切片
- 13.3 HashMap<K, V>
  - 建立與操作
  - Entry API
  - 自訂雜湊函式
- 13.4 其他集合
  - HashSet
  - BTreeMap、BTreeSet
  - VecDeque
  - LinkedList
  - BinaryHeap

### 第 14 章：迭代器

- 14.1 迭代器基礎
  - Iterator trait
  - iter、iter_mut、into_iter
  - next 方法
- 14.2 迭代器適配器
  - map、filter、filter_map
  - take、skip、take_while
  - enumerate、zip
  - chain、flatten、flat_map
  - peekable、fuse
- 14.3 消費者方法
  - collect
  - fold、reduce
  - sum、product
  - any、all
  - find、position
  - max、min
  - count、last
- 14.4 自訂迭代器
  - 實作 Iterator trait
  - IntoIterator trait
  - 雙向迭代器
- 14.5 效能考量
  - 惰性求值
  - 迭代器 vs 迴圈
  - 編譯器最佳化

### 第 15 章：智慧指標

- 15.1 Box<T>
  - 堆積配置
  - 遞迴型別
  - trait 物件
- 15.2 Rc<T> 與 Arc<T>
  - 引用計數
  - 共享所有權
  - Weak<T> 弱引用
  - 迴圈引用問題
- 15.3 RefCell<T> 與 Cell<T>
  - 內部可變性
  - 借用規則執行時期檢查
  - Rc<RefCell<T>> 組合
- 15.4 Cow<T>
  - Clone-on-write
  - 效能最佳化
- 15.5 自訂智慧指標
  - Deref trait
  - DerefMut trait
  - Drop trait

---

## 第四部分：並行與非同步

### 第 16 章：並行程式設計

- 16.1 執行緒基礎
  - std::thread::spawn
  - JoinHandle
  - move 閉包
- 16.2 訊息傳遞
  - Channel（mpsc）
  - Sender 與 Receiver
  - 多生產者
- 16.3 共享狀態
  - Mutex<T>
  - RwLock<T>
  - 死鎖預防
- 16.4 原子操作
  - AtomicBool、AtomicI32 等
  - Ordering
  - 無鎖資料結構
- 16.5 Send 與 Sync
  - 執行緒安全 trait
  - 自動實作規則
  - 手動實作（unsafe）

### 第 17 章：非同步程式設計

- 17.1 非同步基礎
  - async/await 語法
  - Future trait
  - Pin 與 Unpin
- 17.2 非同步執行時
  - tokio
  - async-std
  - smol
  - 執行時比較
- 17.3 Tokio 深入
  - #[tokio::main]
  - spawn 與 JoinHandle
  - select! 巨集
  - 任務取消
- 17.4 非同步 I/O
  - AsyncRead、AsyncWrite
  - 非同步檔案操作
  - 非同步網路
- 17.5 非同步模式
  - join! 並行執行
  - 並行限制（Semaphore）
  - 非同步串流（Stream）
  - 非同步迭代

---

## 第五部分：模組與生態系

### 第 18 章：模組系統

- 18.1 模組基礎
  - mod 關鍵字
  - 檔案即模組
  - 目錄模組
- 18.2 可見性控制
  - pub 公開
  - pub(crate)
  - pub(super)
  - pub(in path)
- 18.3 use 與路徑
  - 絕對路徑 vs 相對路徑
  - use 重新匯出
  - as 別名
  - glob import `*`
- 18.4 專案組織
  - 單一檔案
  - 多檔案
  - 工作空間（Workspace）

### 第 19 章：Cargo 進階

- 19.1 Cargo.toml 詳解
  - [package] 區段
  - [dependencies] 版本語法
  - [dev-dependencies]
  - [build-dependencies]
  - features
- 19.2 工作空間
  - 多專案管理
  - 共享依賴
  - 單一 Cargo.lock
- 19.3 Cargo 擴充
  - cargo-edit
  - cargo-watch
  - cargo-expand
  - cargo-audit
  - cargo-outdated
- 19.4 發布 Crate
  - crates.io 帳號
  - 文件撰寫
  - 版本管理
  - Yanking

### 第 20 章：測試

- 20.1 單元測試
  - #[test] 屬性
  - assert! 巨集系列
  - 測試私有函式
  - 測試組織
- 20.2 整合測試
  - tests/ 目錄
  - 測試公開 API
  - 共用測試工具
- 20.3 文件測試
  - 文件註解中的程式碼
  - 自動執行範例
- 20.4 進階測試
  - 自訂測試框架
  - 基準測試（Benchmark）
  - 模糊測試（Fuzzing）
  - 屬性測試（Property-based testing）

### 第 21 章：文件與註解

- 21.1 註解類型
  - 單行註解 //
  - 區塊註解 /**/
  - 文件註解 ///
  - 模組文件 //!
- 21.2 rustdoc
  - Markdown 支援
  - 程式碼範例
  - 跨引用連結
  - 屬性標記
- 21.3 文件最佳實踐
  - 公開 API 文件
  - 範例程式碼
  - 錯誤說明

---

## 第六部分：實戰應用

### 第 22 章：命令列工具

- 22.1 參數解析
  - std::env::args
  - clap crate
  - structopt（已合併入 clap）
- 22.2 終端機 I/O
  - 標準輸入輸出
  - 彩色輸出
  - 進度條
- 22.3 設定檔處理
  - 環境變數
  - TOML、YAML、JSON
  - 設定檔路徑

### 第 23 章：網路程式設計

- 23.1 HTTP 客戶端
  - reqwest 基礎
  - 非同步請求
  - 錯誤處理
  - 重試機制
- 23.2 HTTP 伺服器
  - axum 框架
  - actix-web 框架
  - 路由設計
  - 中介軟體
- 23.3 其他協定
  - WebSocket
  - gRPC（tonic）
  - TCP/UDP

### 第 24 章：資料庫操作

- 24.1 SQL 資料庫
  - sqlx
  - diesel
  - 連線池
- 24.2 NoSQL 資料庫
  - Redis
  - MongoDB
- 24.3 ORM 模式
  - 模型定義
  - 遷移（Migration）
  - 查詢建構器

### 第 25 章：序列化與反序列化

- 25.1 Serde 基礎
  - Serialize trait
  - Deserialize trait
  - derive 巨集
- 25.2 常見格式
  - JSON（serde_json）
  - TOML（toml）
  - YAML（serde_yaml）
  - MessagePack、CBOR
- 25.3 自訂序列化
  - 欄位屬性
  - 自訂序列化器
  - 處理複雜型別

### 第 26 章：WebAssembly

- 26.1 WASM 基礎
  - 什麼是 WebAssembly
  - Rust 與 WASM
  - wasm-pack
- 26.2 與 JavaScript 互動
  - wasm-bindgen
  - 匯出函式
  - 處理複雜型別
- 26.3 WASM 應用
  - 瀏覽器應用
  - Node.js 整合
  - Serverless 函式

---

## 第七部分：進階主題

### 第 27 章：巨集系統

- 27.1 宣告式巨集
  - macro_rules! 語法
  - 模式匹配
  - 重複模式
  - 衛生性（Hygiene）
- 27.2 程序式巨集
  - derive 巨集
  - 屬性巨集
  - 函式式巨集
- 27.3 常用巨集模式
  - 建構器模式
  - DSL 設計
  - 程式碼生成

### 第 28 章：Unsafe Rust

- 28.1 Unsafe 基礎
  - 為什麼需要 unsafe
  - unsafe 區塊
  - unsafe 函式
- 28.2 Unsafe 能力
  - 解引用裸指標
  - 呼叫 unsafe 函式
  - 存取可變靜態變數
  - 實作 unsafe trait
- 28.3 FFI（外部函式介面）
  - 呼叫 C 函式
  - 被 C 呼叫
  - bindgen 工具
  - cbindgen 工具
- 28.4 安全抽象
  - 封裝 unsafe 程式碼
  - 安全 API 設計
  - 文件與測試

### 第 29 章：效能最佳化

- 29.1 基準測試
  - criterion crate
  - 微基準測試
  - 效能回歸測試
- 29.2 分析工具
  - perf
  - flamegraph
  - valgrind
  - heaptrack
- 29.3 最佳化技巧
  - 編譯器最佳化等級
  - 內聯提示
  - SIMD
  - 記憶體配置策略
- 29.4 常見效能陷阱
  - 不必要的複製
  - 過度配置
  - 錯誤的資料結構選擇

### 第 30 章：設計模式

- 30.1 創建型模式
  - 建構器模式（Builder）
  - 工廠模式
  - 單例模式（謹慎使用）
- 30.2 結構型模式
  - 適配器模式
  - 裝飾器模式
  - 外觀模式
- 30.3 行為型模式
  - 策略模式
  - 觀察者模式
  - 狀態模式
- 30.4 Rust 特有模式
  - Newtype 模式
  - Type State 模式
  - RAII 模式

---

## 附錄

### 附錄 A：常用 Crate 推薦

- 錯誤處理：anyhow、thiserror
- 序列化：serde、serde_json
- HTTP：reqwest、axum、actix-web
- 非同步：tokio、async-std
- CLI：clap、dialoguer
- 日誌：tracing、log
- 日期時間：chrono、time
- 正則表達式：regex
- 隨機數：rand

### 附錄 B：Rust 版本演進

- Rust 2015
- Rust 2018
- Rust 2021
- Rust 2024

### 附錄 C：常見錯誤與解決方案

- 借用檢查器錯誤
- 生命週期錯誤
- trait 約束錯誤
- 非同步錯誤

### 附錄 D：Rust 速查表

- 語法速查
- 常用 trait
- 迭代器方法
- 字串處理

### 附錄 E：學習資源

- 官方資源
- 書籍推薦
- 線上課程
- 社群

---

## 學習路徑建議

```txt
入門者（1-2 週）
├── 第 1-5 章：基礎語法
└── 第 6-7 章：所有權（核心！）

初級（2-4 週）
├── 第 8-10 章：結構體、列舉、錯誤處理
├── 第 11-12 章：泛型與 Trait
└── 第 13-14 章：集合與迭代器

中級（4-8 週）
├── 第 15 章：智慧指標
├── 第 16-17 章：並行與非同步
├── 第 18-21 章：模組、測試、文件
└── 第 22-26 章：實戰應用

進階（持續學習）
├── 第 27-28 章：巨集與 Unsafe
├── 第 29-30 章：效能與設計模式
└── 深入特定領域（Web、嵌入式、遊戲等）
```

---

最後更新：2026-01-17
