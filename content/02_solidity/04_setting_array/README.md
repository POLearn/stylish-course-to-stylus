# 向数组中添加元素

现在我们的合约已经部署完毕，是时候使用 `push` 函数**向动态数组添加值**了。这个函数允许我们将新元素插入到 `arr` 数组中。

让我们快速回顾一下。

```rust
pub fn push(&mut self, i: U256) {
    self.arr.push(i);
}
```

- **`i`**: 要添加到数组中的 `U256` 值。

`.push()` 函数通过在数组末尾添加新元素来动态增加数组的大小。由于 `arr` 是一个**动态数组**，我们可以继续添加元素，而无需预定义大小。

### 任务：调用 `push` 函数

为了完成这个任务，调用 `push` 方法，并使用值 `42161`（一个 `U256` 值）。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_push.png)

你可以通过使用 `get_element`（索引为 0）来验证插入结果，并通过 `get_arr_length`（应该为 1）来检查数组的长度。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_get.png)

最后，提交交易到 Proof of Learn 以完成任务。这将帮助你与动态数组交互并修改合约状态。