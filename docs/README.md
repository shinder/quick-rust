# JavaScript 工程師的 Rust 快速入門指南

> 本指南專為有 JavaScript 背景的開發者設計，透過對比 JS 與 Rust 的差異，幫助你快速掌握 Rust 核心概念。

## 目錄

### [第一章：環境設定與 Hello World](./01-getting-started.md)

- 安裝 Rust (rustup)
- Cargo 套件管理工具（類似 npm）
- 建立第一個專案
- 編譯與執行
- VS Code 設定建議

### [第二章：基本語法與型別系統](./02-basics.md)

- 變數宣告（let vs const vs mut）
- 基本型別（數字、字串、布林）
- 函式定義
- 控制流程（if、loop、match）
- JS vs Rust 語法對照表

### [第三章：所有權與借用](./03-ownership.md)

- 所有權概念（Rust 最獨特的特性）
- 移動語義 vs JS 的參考傳遞
- 借用與引用（& 和 &mut）
- 生命週期入門
- 常見錯誤與解決方式

### [第四章：結構體與列舉](./04-structs-enums.md)

- 結構體（類似 JS 的 class/object）
- 實作方法（impl）
- 列舉與模式匹配
- Option 與 Result（取代 null/undefined）
- 從 JS 物件思維轉換到 Rust

### [第五章：錯誤處理](./05-error-handling.md)

- Result 與 Option 深入
- ? 運算子（優雅的錯誤傳遞）
- panic! 與 unwrap
- 自訂錯誤類型
- 對比 JS 的 try-catch

### [第六章：集合與迭代器](./06-collections.md)

- Vec（類似 JS Array）
- HashMap（類似 JS Object/Map）
- 迭代器與閉包
- map、filter、fold（類似 JS 陣列方法）
- 鏈式呼叫與惰性求值

### [第七章：模組與套件管理](./07-modules.md)

- 模組系統（mod、pub、use）
- Cargo.toml（類似 package.json）
- crates.io（類似 npm）
- 常用 crate 推薦
- 專案結構最佳實踐

### [第八章：非同步程式設計](./08-async.md)

- async/await（與 JS 很相似！）
- Future 與 Promise 的對比
- tokio 執行時
- 非同步 HTTP 請求範例
- 常見的非同步陷阱

---

## 學習建議

1. **動手實作**：每章都有練習題，務必親自敲過程式碼
2. **善用編譯器**：Rust 編譯器的錯誤訊息非常友善，是最好的老師
3. **循序漸進**：第三章「所有權」是關鍵，務必完全理解再往下
4. **對比思考**：持續用 JS 的思維來理解 Rust，找出相似與差異

## 為什麼 JS 工程師該學 Rust？

| 特點       | JavaScript            | Rust                       |
| ---------- | --------------------- | -------------------------- |
| 執行速度   | 中等（V8 最佳化）     | 極快（接近 C/C++）         |
| 記憶體安全 | GC 自動管理           | 編譯時期保證，零成本       |
| 型別系統   | 動態弱型別            | 靜態強型別                 |
| 空值處理   | null/undefined        | Option 類型，編譯時檢查    |
| 並行處理   | 單執行緒 + Event Loop | 多執行緒，無資料競爭       |
| 應用場景   | Web 前後端            | 系統程式、WebAssembly、CLI |

## 開始學習

準備好了嗎？讓我們從[第一章：環境設定與 Hello World](./01-getting-started.md) 開始吧！
