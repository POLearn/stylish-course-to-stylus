# Stylus 智能合约项目结构  

一个典型的 **Stylus 合约** 由两个主要的 Rust 文件组成：`lib.rs` 和 `main.rs`。每个文件承担不同的角色，共同构建完整的智能合约。  

---

## `lib.rs` – 核心合约逻辑  

`lib.rs` 主要定义智能合约的核心逻辑。这个文件包含 **业务逻辑**，包括数据结构、合约入口点以及与区块链交互的函数。  

从之前的 **Ownable** 智能合约示例可以看出，`lib.rs` 的结构通常包含：  

- **`#[storage]`**：标记 `Ownable` 结构体为存储结构，持久化合约状态（例如 `owner` 变量）。  
- **`#[entrypoint]`**：定义合约的 **入口点**，标记可以被外部调用的函数。  
- **`#[external]`**：标记外部可访问的函数，如 `owner`（获取所有者）和 `set_owner`（设置所有者）。  

大部分合约逻辑都在这里定义，包括存储变量、状态修改逻辑以及与区块链的交互。  

---

## `main.rs` – 初始化和 ABI 导出  

`main.rs` 主要负责合约 **初始化**，确保合约能够正确执行和部署。此外，它还管理 ABI（应用二进制接口）导出等功能。  

### `main.rs` 结构示例  

```rust
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[cfg(not(any(test, feature = "export-abi")))]
#[no_mangle]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
```

### 关键部分解析  

- **`#[cfg]`**：条件编译，确保 `export-abi` 功能启用时可以导出 ABI，否则默认 `main` 为空。  
- **`no_main`**：如果没有启用 `export-abi`，就不会编译默认的 `main` 函数，减少合约大小。  
- **`print_abi`**：当 `export-abi` 启用时，打印合约 ABI，方便与 Solidity 等其他语言交互。  

`lib.rs` 负责合约核心逻辑，而 `main.rs` 主要用于 **初始化、管理 ABI 以及部署前的设置**。  

---

## `Cargo.toml` – Rust 项目配置  

`Cargo.toml` 是 **Stylus 智能合约的项目配置文件**，用于定义项目的元数据、依赖项以及构建配置。  

### 关键内容  

```toml
[package]
name = "stylus_contract"
version = "0.1.0"
edition = "2021"

[dependencies]
stylus-sdk = "0.1"
alloy-primitives = "0.2"
mini-alloc = "0.4"

[features]
export-abi = []
debug = []
```

### 重要部分解析  

- **`[dependencies]`**：  
  - `stylus-sdk`：Stylus 合约开发的核心库。  
  - `alloy-primitives`：提供以太坊兼容的类型（如 `Address`）。  
  - `mini-alloc`：轻量级内存管理，提高智能合约执行效率。  

- **`[features]`**：  
  - `export-abi`：启用 ABI 导出功能，方便与外部系统交互。  
  - `debug`：调试模式，便于开发过程中进行日志记录和错误排查。  

---

## 总结  

一个完整的 **Stylus 智能合约** 由以下三部分组成：  

1. **`lib.rs`** - **核心合约逻辑**（数据存储、状态管理、业务逻辑）。  
2. **`main.rs`** - **初始化与 ABI 导出**（合约执行管理、部署前设置）。  
3. **`Cargo.toml`** - **项目配置**（定义元数据、依赖项、构建功能）。  

这套结构让 **Stylus 智能合约既保持了 Rust 代码的可读性，又兼具 Solidity 开发的易用性**，使 Solidity 开发者可以更轻松地过渡到 Rust 生态中的智能合约开发。 🚀