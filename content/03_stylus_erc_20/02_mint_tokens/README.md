# 任务：铸造你的代币！🚀

现在你的 Stylus ERC20 合约已经部署完成，你可以像在以太坊或 Base 上的 ERC20 代币一样与它进行交互。但这里有个小惊喜——这个代币是使用 Rust 构建的，并且运行在一个像 EDU Chain 这样的 EVM 兼容链上。酷吧？🎉

在本节中，我们将带你完成铸造简单的 `1000000` 个代币并将它们发送到你自己的地址的过程。让我们分解一下它是如何工作的。下面的代码是已部署合约中的 `mint` 函数，它允许我们创建新代币并将它们分配给一个地址。该函数接受一个 `value` 参数（在我们的例子中是 `1000000` 个代币），并调用我们之前定义的 ERC20 `mint` 方法。它使用 `msg::sender()` 来获取发起铸造请求的地址，并将代币分配给该地址。简单来说，这个函数使我们能够生成新代币并将其发送给调用该函数的用户。

```rust
/// 铸造代币
pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
    self.erc20.mint(msg::sender(), value)?;
    Ok(())
}
```

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_mint.png)  

一旦你铸造了代币，别忘了将交易提交到 Proof of Learn 并展示你的进度！

铸造代币是创建新代币并将它们发送到特定地址的过程。通过 ERC20 代币，这个操作不仅会将代币添加到账户中，还会解锁 ERC20 代币的所有酷炫功能，比如转账、检查余额等。要查看你的代币余额，只需调用 `balanceOf` 并提供铸造代币的地址（或者任何地址）！

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_balanceof.png)  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_balance.png)