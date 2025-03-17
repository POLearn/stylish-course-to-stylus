# 调用智能合约方法

在本指南中，我们将向您介绍如何调用 `init` 方法，这是合约中的一个写方法。写方法是修改或设置合约状态的函数，例如更改变量、更新值或执行会改变区块链状态的操作。

## init

`init` 方法通常用于初始化合约的状态。在合约示例中，它设置了 `owner` 和 `max_supply` 变量。它通常作为一个一次性设置函数，仅在合约首次部署或第一次调用时运行。

## 任务：调用 `init` 方法

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract_init.png)  

在智能合约部署后，您可以调用 `init` 方法。该方法是一个写操作，这意味着它将修改合约的状态。您通常通过一个广播到网络的交易来调用 `init` 方法。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract_init_success.png)  

现在，您可以调用其他方法，如 `owner` 和 `max` 来查看这些值。我们将在下一部分讨论如何调用智能合约的方法。