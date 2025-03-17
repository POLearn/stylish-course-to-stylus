## Stylus 与 Solidity 变量的等效

在我们开始了解如何使用和部署 Stylus 合约之前，让我们先简单地看看 Solidity 中的全局变量。例如 `block` 和 `msg` 等。这些非常有用，并且在 Stylus SDK 中也有对应的变量。

以 Solidity 中非常重要的 `msg.sender()` 为例，在 Stylus 中它是：

```rust
use stylus_sdk::{msg};

let address = msg::sender();
```

## Stylus 中 msg 的等效

`use stylus_sdk::{msg};`

- msg::reentrant() → 检查当前调用是否为重入调用
- msg::sender() → 获取调用合约的账户地址（相当于 Solidity 的 msg.sender）
- msg::value() → 返回发送到合约的 ETH 值（以 wei 为单位）（相当于 Solidity 的 msg.value）

## Stylus 中 block 的等效

`use stylus_sdk::{block};`

- block::basefee() → 获取当前区块交易所需的最低 gas 费用
- block::chainid() → 返回合约所在的 Arbitrum 链的唯一标识符
- block::coinbase() → 获取负责发布 L1 批次的实体地址（类似于 Solidity 中的 block.coinbase，但特定于 Arbitrum）
- block::gas_limit() → 获取区块内事务允许的最大 gas 限制
- block::number() → 提供一个估算的 L1 区块号，表示交易被 sequencer 包含的区块
- block::timestamp() → 返回一个估算的 Unix 时间戳，表示 sequencer 处理事务的时间

## Stylus 中的 keccak

`use stylus_sdk::{crypto};`

- crypto::keccak: 高效地计算给定预图像的 keccak256 哈希值

这些只是 Stylus 中的一些全局变量，用于访问事务和区块数据。Stylus 提供了额外的内置功能，用于与虚拟机交互，例如检索 gas 详情、访问执行上下文和处理加密操作。有关可用功能的完整列表，请查阅官方的 [Stylus VM 功能文档](https://docs.arbitrum.io/stylus-by-example/basic_examples/vm_affordances)。🚀

## 加载智能合约

让我们将这个合约加载到 IDE 或所需的环境中。

`https://github.com/polearn/stylish-course-to-stylus/tree/master/contract/variable`

这个 Rust 结构体，标记为 `#[storage]`，定义了 Stylus 智能合约的持久化存储变量，类似于 Solidity 中的状态变量。`initialized` 字段（`StorageBool`）跟踪合约是否已经初始化，类似于 Solidity 中的 `bool`。`owner` 字段（`StorageAddress`）存储合约所有者的地址，类似于 Solidity 中的 `address`。最后，`max_supply`（`StorageU256`）存储一个无符号整数值，类似于 Solidity 中的 `uint256`。这些变量存储在链上，并在事务之间保持持久化，类似于 Solidity 的状态变量。

```rust
#[storage]
#[entrypoint]
pub struct Contract {
    initialized: StorageBool,
    owner: StorageAddress,
    max_supply: StorageU256,
}
```

### #[entrypoint]

另一个重要的宏是 `#[entrypoint]`。这是 Stylus 合约的必需宏，作为 Stylus 合约执行的起始点，确保合约知道如何以及在哪里编译和 **检查**。通常应用于主存储结构，使其公共方法成为主要的调用方法。每个合约只能有一个入口点，防止多个顶级执行路径。

### init()

让我们看看 `public fn init` 方法。这是一个常见的无构造函数的方法，用于初始化合约。它包含一个布尔值，以确保初始化过程不会多次运行。更重要的是，变量是如何设置的。

要在合约中设置 `owner` 和 `max_supply` 状态变量，可以使用 `msg::sender()` 和 `block::timestamp()` 全局变量。要初始化 `owner`，使用 `self.owner.set(msg::sender())` 将其设置为调用合约的账户地址。类似地，要设置 `max_supply`，使用 `self.max_supply.set(U256::from(block::timestamp()))` 将其设置为当前区块的时间戳。

这些全局变量为合约提供了重要的上下文信息，如调用者的地址和区块的时间戳，模仿了 Solidity 中的类似功能。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_set.png)

现在准备好了。让我们编译

> ❗请注意，在 Stylide IDE 中编译可能需要一分钟时间。请耐心等待，直到合约被调用和检查。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_compile.png)

### 激活合约

如果你收到了 Metamask，这完全没问题。在 Stylus 中，有一个叫做 Activate Program 的功能，它通常在大多数合约中启用，以提高部署效率并使 Stylus 合约可用。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_deploy.png)

### 任务：提交部署

找到部署事务，通常可以在 Metamask 中找到，或者通过在浏览器中搜索你的合约。获取该事务并提交到 Proof of Learn。恭喜你，你已经成功部署了 Stylus 合约！

你可以看到我们讨论过的方法，如 `init()`，我们将在接下来的内容中调用它们。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_deployed.png)