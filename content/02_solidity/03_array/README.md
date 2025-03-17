# 数组在 Stylus 中

与 Solidity 类似，**数组**可以是 **固定大小**（在编译时确定）或 **动态大小**（在运行时增长或缩小）。在 Stylus 中，数组遵循 Rust 的存储原则，同时保持 Solidity 类似的可用性。

## Solidity 中的数组

Solidity 中的数组用于存储相同类型的多个元素，可以是 **固定大小** 或 **动态大小**。固定大小数组的长度是预定的，而动态数组可以在运行时增长或缩小。

Solidity 中的数组通过 `type[]` 语法声明动态数组，或者通过 `type[size]` 语法声明固定大小数组。数组可以包含任何有效的 Solidity 数据类型，包括原始类型（`uint`、`address`、`bool`）、结构体，甚至是其他数组。

```solidity
// Solidity 数组示例

// 动态数组（大小可以变化）
uint256[] public dynamicArray;

// 固定大小数组（大小为 5）
uint256[5] public fixedArray;
```

## Stylus 中的数组

在 Stylus 中，数组遵循类似的概念，但使用 Rust 的语法实现。Stylus 提供了 `StorageVec<T>` 来表示 **动态数组**，而固定大小的数组使用 `[T; N]` 来声明。两种类型都利用了 Rust 高效的内存管理，同时保持了 Solidity 类似的行为。

```rust
// Stylus Rust 数组示例
#[storage]
pub struct Contract {
    uint256[3] arr2;
}
```

### 差异与相似之处

| 特性         | Solidity                    | Stylus (Rust)               |
|--------------|-----------------------------|-----------------------------|
| 语法         | `type[]` 或 `type[size]`      | `StorageVec<T>` 或 `[T; N]`  |
| 大小         | 固定或动态                  | 固定或动态                  |
| 可迭代性     | 支持循环                    | 支持循环                    |
| 存储类型     | 存储在合约存储中            | 动态使用 `StorageVec`，固定使用 `[T; N]` |

### 数组合约

在本节中，我们将深入探讨一个简单的数组功能合约。你可以在 [GitHub 仓库](https://github.com/POLearn/stylish-course-to-stylus/tree/master/contract/array) 获取完整的源代码并在 IDE 中加载。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_contract.png)

该合约包含一个 **动态数组** `arr`。与固定大小数组不同，动态数组可以在运行时增长或缩小，使它们在存储多个值时更加灵活。

```rust
sol_storage! {
    #[entrypoint]
    pub struct Contract {
        uint256[] arr;
    }
}
```

我们可以通过使用内置方法来高效地管理和操作存储的数据。

```rust
// 向动态数组添加元素
pub fn push(&mut self, i: U256) {
    self.arr.push(i);
}

// 获取数组中指定索引的元素
pub fn get_element(&self, index: U256) -> U256 {
    self.arr.get(index).unwrap()
}

// 获取数组的长度
pub fn get_arr_length(&self) -> U256 {
    U256::from(self.arr.len())
}

// 移除元素（不会改变数组的大小）
pub fn remove(&mut self, index: U256) {
    let mut last_element = self.arr.setter(index).unwrap();
    last_element.erase()
}
```

- **`push`**：该函数通过 `.push(i)` 向动态数组添加新元素。由于 Stylus 数组是可调整大小的，因此可以在运行时添加新值。
  
- **`get_element`**：使用 `.get(index)` 获取指定索引处的值。`.unwrap()` 确保返回有效值，防止索引越界错误。
  
- **`get_arr_length`**：返回数组的元素总数。`.len()` 获取数组的长度，然后转换为 `U256` 以匹配 Solidity 的行为。
  
- **`remove`**：该函数使用 `.setter(index).erase()` 移除指定索引处的元素。然而，与 Solidity 的 `pop()` 不同，这不会缩小数组大小；它只是清除给定索引处的值。

### 任务：编译与部署

现在，让我们**编译**并**部署**合约。

![编译](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_compile.png)

![部署](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_deploy.png)

部署后，可能需要**激活**合约，这可能会触发第二个交易。一旦完成，你就可以开始使用了！

接下来，我们将继续**设置（或者更确切地说是推送）元素到合约数组中**。别忘了**提交你的交易到 Proof of Learn**！🚀

你还可以查看一些实际示例：  
🔗 [Arbitrum Stylus 示例 – 数组](https://arbitrum-stylus-by-example.vercel.app/basic_examples/arrays)