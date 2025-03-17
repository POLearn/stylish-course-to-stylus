# 设置 setMyNestedMap

现在我们已经部署了合约，接下来是使用 `setMyNestedMap` 函数在嵌套映射中**设置值**。这个函数允许我们更新 `my_nested_map` 中的特定条目。

让我们回顾一下函数定义。

```rust
pub fn set_my_nested_map(&mut self, index: U256, target: Address, new_value: bool) {
    self.my_nested_map.setter(index).setter(target).set(new_value);
}
```

- **`index`**：这是主键（`U256`），用于标识外部映射。
- **`target`**：你正在交互的账户的地址（`Address`）。
- **`new_value`**：你想要设置的新布尔值。

`setter()` 函数是导航嵌套结构的关键。首先，使用 `setter(index)` 访问与给定 `index` 关联的嵌套映射。然后，调用 `setter(target)` 到达特定地址。最后，使用 `.set(new_value)` 来更新值。

### 任务：调用 `setMyNestedMap`

为了完成这个任务，调用 `setMyNestedMap` 方法，传入以下值：将 **`index`** 设置为 `10`（一个 `U256` 键），提供一个有效的 **`target`** 地址（可以使用你自己的地址），并将 **`new_value`** 设置为 `true`（一个布尔值）。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_set.png)  

你可以通过调用 `getMyNestedMap` 来测试该值。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_value.png)

最后，将交易提交到 Proof of Learn，以完成任务。这将帮助你与嵌套映射交互并修改合约状态。