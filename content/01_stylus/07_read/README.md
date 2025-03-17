# 从合约读取数据

就像在 Solidity 中一样，从合约读取数据允许你获取合约状态变量的当前值。如果你查看合约的其他方法，主要是：

```rust
pub fn owner(&self) -> Address {
    self.owner.get()
}

pub fn max(&self) -> U256 {
    self.max_supply.get()
}
```

这些值方法可以作为 getter 方法，允许我们检查合约的状态，这些信息可能非常重要，比如谁拥有合约，或者一个代币的最大供应量是多少。记住，这些值是在之前调用的 `init` 方法中设置的。

## 任务：带有变数的读取

对于这个任务，让我们改变一下方式。加载合约。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_load.png)  

然后，像调用 `init` 一样，你可以点击 `max` 方法来调用或**读取**合约的值。在控制台中，你应该会从合约中得到一个响应，通常是该合约被调用时的时间戳。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_load.png)  

现在，将该值放入双引号中，如 `"ANSWER"`，并提交到 Proof of Learn，完成此任务并获得奖励。