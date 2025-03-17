## 在 Stylus 中理解存储  

Stylus Rust SDK 的存储机制与 Solidity 类似，使智能合约能够在交易之间持久化数据。然而，由于 Rust 不是为区块链原生设计的，Stylus 引入了特定的存储类型，以高效地处理链上数据。  

## `#[storage]` 宏  

`#[storage]` 宏将 Rust 结构体标记为持久化存储。结构体中的每个字段都必须使用实现 `StorageType` 特性的专用存储类型。这些存储类型确保数据能够被高效地存储和检索。  

```rust
   #[storage]
   pub struct Contract {
       owner: StorageAddress,  // 存储以太坊地址
       active: StorageBool,    // 存储布尔值
       sub_struct: SubStruct,  // 具有存储特性的嵌套结构体
   }

   #[storage]
   pub struct SubStruct {
       // 其他使用 StorageType 的字段
   }
   ```  

你可以在 `use stylus_sdk::storage` 模块中找到这些存储类型，包括：  

```rust
use stylus_sdk::storage::{
    StorageAddress, // 存储以太坊地址
    StorageBool,    // 存储布尔值
    StorageU256,    // 存储 256 位无符号整数（等同于 Solidity 的 uint256）
    StorageString,  // 存储类似 Solidity 字符串的值
    StorageMap,     // 存储键值映射，类似于 Solidity 的 mapping()
    StorageVec      // 存储动态数组，类似于 Solidity 的数组类型
};
```  

## `sol_storage!` 宏  

如果你更熟悉 Solidity 的语法，可以使用 `sol_storage!` 宏来定义存储结构，该宏会自动将 Solidity 风格的定义转换为 Rust 等效形式。这些类型可以通过 `use stylus_sdk::{prelude::*}` 引入。  

```rust
   sol_storage! {
       pub struct Contract {
           address owner;      // 转换为 StorageAddress
           bool active;        // 转换为 StorageBool
           SubStruct sub_struct;
       }

       pub struct SubStruct {
           mapping(address => uint) balances;  // 转换为 StorageMap
           Delegate delegates[];               // 转换为 StorageVec
       }
   }
```  

这种方法可以更轻松地将 Solidity 合约迁移到 Rust，同时保持完全一致的存储布局。由于存储映射相同，从 Solidity 迁移到 Stylus Rust 十分顺畅——只需复制粘贴你的类型定义即可！  

想要了解更多关于 Stylus Rust SDK 存储的内容，请查看官方文档：  
👉 [Stylus Rust SDK 存储指南](https://docs.arbitrum.io/stylus/reference/rust-sdk-guide#storage)  