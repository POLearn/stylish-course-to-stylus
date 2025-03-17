# 映射  

映射是 Solidity 中的基本数据结构，作为哈希表存储键值对。它们通常用于高效存储和检索数据，例如跟踪余额或所有权。Stylus 是一个基于 Rust 的 Arbitrum 智能合约框架，它使用 `StorageMap` 提供了类似的概念。  

## Solidity 中的映射

在 Solidity 中，映射使用 `mapping(keyType => valueType)` 语法声明。键可以是任何内建的值类型（例如 `uint`、`address`、`bytes`），值可以是任何类型，包括另一个映射或数组。然而，Solidity 中的映射是不可迭代的，意味着不能直接对它们进行循环遍历。  

```solidity
// Solidity 映射示例
mapping(address => uint256) public balances;
```

## Stylus 中的映射

在 Stylus 中，映射通过 `StorageMap<keyType, StorageType>` 来实现，遵循 Rust 的语法。`keyType` 必须是来自 `alloy_primitives` 的类型，而 `valueType` 必须是支持的 `StorageType`。这种结构确保了基于 Rust 的智能合约中高效的键值存储。  

```rust
// Stylus Rust 映射示例
#[storage]
pub struct Contract {
    balances: StorageMap<Address, StorageU256>,
}
```

### 差异与相似性

| 特性          | Solidity                     | Stylus (Rust)              |
|---------------|-----------------------------|----------------------------|
| 语法          | `mapping(keyType => valueType)` | `StorageMap<keyType, StorageType>` |
| 键类型        | 内建类型，`bytes`，`string`，合约地址 | 来自 `alloy_primitives` 的类型 |
| 值类型        | 任何类型，包括其他映射或数组 | 任何 `StorageType` |
| 可迭代性      | 不可迭代                     | 不可迭代                   |

### 映射合约

在这一部分，我们将深入探讨 Stylus 中的映射和嵌套映射。你可以在这里访问完整的源代码：  
👉 [GitHub 仓库](https://github.com/POLearn/stylish-course-to-stylus/tree/master/contract/mapping)  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_contract.png)

```rust
sol_storage! {
    #[entrypoint]
    pub struct Contract {
        mapping(address => bool) my_map;
        mapping(uint256 => mapping(address => bool)) my_nested_map;
    }
}
```  

这是合约的 *入口点*。如你所记得，`sol_storage!` 定义了合约的存储结构，允许我们创建类似 Solidity 的映射变量，同时利用 Rust 的语法。  

- **`my_map`**：一个简单的映射，将 `address` 与 `bool` 关联，适用于跟踪权限、白名单或其他标志。  
- **`my_nested_map`**：一个 **嵌套映射**，其中 `uint256` 键（例如 ID）映射到另一个 `mapping(address => bool)`。这种结构通常用于跟踪审批、基于角色的访问或多级权限。  

现在，让我们探索如何在 Stylus 中与 **嵌套映射** 进行交互。以下函数允许我们 **读取**、**更新** 和 **删除** `my_nested_map` 中的值。  

```rust
pub fn get_my_nested_map(&self, index: U256, target: Address) -> bool {
    self.my_nested_map.get(index).get(target)
}

// 更新该地址的值
pub fn set_my_nested_map(&mut self, index: U256, target: Address, new_value: bool) {
    self.my_nested_map.setter(index).setter(target).set(new_value);
}

// 将值重置为默认值。
pub fn remove_my_nested_map(&mut self, index: U256, target: Address) {
    self.my_nested_map.setter(index).delete(target);
}
```

- **`get_my_nested_map`**：这个函数使用 `.get()` 方法检索存储的值。首先，它访问特定 `index`（一个 `U256` 键）下的映射，然后检索与 `target` 地址相关联的布尔值。这对于检查权限或跟踪审批非常有用。  

- **`set_my_nested_map`**：要更新或插入值，使用 `.setter()` 函数导航嵌套映射，然后调用 `.set(new_value)`。第一个 `.setter(index)` 访问 `index` 下的映射，而第二个 `.setter(target)` 访问特定地址。最后，`.set(new_value)` 更新布尔值。  

- **`remove_my_nested_map`**：`.delete()` 函数重置映射条目，有效地移除存储的数据。通过使用 `.setter(index).delete(target)`，我们清除了给定 `index` 下的 `target` 的映射，这对于撤销权限或重置值非常有用。  

现在你理解了嵌套映射的工作原理，`my_map` 功能是同一概念的 **简化版**。它使用相同的方法——`.get()`、`.setter()`、`.set()` 和 `.delete()`——来对 **单层映射** 执行 **CRUD（创建、读取、更新、删除）** 操作，而不是嵌套映射。

### 任务：编译与部署  

现在，让我们 **编译** 并 **部署** 合约。  

![编译](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_compile.png)  

![部署](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_deploy.png)  

部署后，你可能需要 **激活** 合约，这可能会触发第二次交易。完成后，你就准备好了！  

接下来，我们将继续 **设置嵌套映射中的值**。同时，别忘了 **提交你的交易到 Proof of Learn！** 🚀  

你还可以在这里查看实际示例：
🔗 Arbitrum Stylus 通过示例 – 映射