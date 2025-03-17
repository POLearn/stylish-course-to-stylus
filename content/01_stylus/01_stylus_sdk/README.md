# Stylus SDK  

Stylus SDK 是一个强大的工具包，专为使用 Rust 及其他 WASM 兼容语言开发智能合约而设计。它构建在 **Alloy** 之上，Alloy 是一个用于以太坊开发的 Rust 库，使开发者能够轻松在 Arbitrum 上创建高性能的去中心化应用（dApps）。Stylus SDK 可与现有的 Rust 库无缝集成，并提供一系列功能，以支持高效的合约开发、部署和交互。  

对于来自 Solidity 生态的开发者而言，Stylus SDK 提供了熟悉的编程风格，这是一大优势。它让 Solidity 开发者能够快速理解并使用 Rust，从而减少学习全新语言的时间成本。此外，Stylus SDK 与 Solidity 共享相似的编程概念，使开发者能够在 Stylus SDK 中同时利用 Rust 和 Solidity 的经验。  

## Solidity 智能合约  

如果你熟悉 Solidity，以下是一个基本的 **Ownable** 合约，它通过地址管理所有权。该合约包含两个主要函数：一个用于获取当前所有者地址，另一个用于更新所有者地址。它使用一个 **公共状态变量** 存储所有者地址，以便系统中的其他部分可以交互或修改所有权。  

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

contract Ownable {
    address public owner;

    function owner() public pure returns(address) {
        return owner;
    }

    function setOwner(address _owner) public {
        owner = _owner;
    }
}
```

## Stylus Rust SDK 智能合约  

在 Stylus SDK 中，**Ownable** 智能合约的核心逻辑与 Solidity 版本相似，仍然是通过地址管理所有权。在这个 Stylus 合约中，`owner` 使用 `StorageAddress` 类型存储，这是 Stylus 中的一个特殊类型，能够将所有者地址持久化存储在链上。  

- `owner()` 函数类似于 Solidity 合约的 **getter**，用于检索当前所有者地址。  
- `set_owner()` 函数允许更新所有者地址，对应于 Solidity 中的 `setOwner()`。  

```rust
use stylus_sdk::alloy_primitives::{Address};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress6};

#[storage]
#[entrypoint]
pub struct Ownable {
    owner: StorageAddress,
}

#[external]
impl Ownable {
    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    pub fn set_owner(&self, _owner: StorageAddress) {
        self.owner.set(_owner);
    }
}
```

### 关键区别  

- **Rust 语法**：Stylus 使用 Rust 编写智能合约，与 Solidity 语法不同。  
- **存储声明**：`#[storage]` 标记 `owner` 为存储变量，使其持久化存储在链上。  
- **入口点声明**：`#[entrypoint]` 指定合约的入口点。  
- **存储类型**：Stylus 使用 `StorageAddress` 存储地址，而 Solidity 直接使用 `address` 类型。  

尽管语法不同，Stylus 仍然保留了 Solidity 的 **核心所有权管理逻辑**。这展示了 Stylus 如何使开发者能够使用 Rust 编写智能合约，同时保持 Solidity 的熟悉模式，从而让 Solidity 开发者更容易过渡到 Rust 生态的智能合约开发。