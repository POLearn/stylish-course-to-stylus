# ERC-20 在 Stylus 中

在 Solidity 中，**ERC-20** 标准定义了一套用于创建可替代代币的通用规则。这些代币广泛应用于去中心化应用（dApps）中，用于表示资产、货币。ERC-20 合规性：合约实现了标准功能，如 `total_supply`、`balance_of`、`transfer`、`approve` 和 `transfer_from`，确保与现有的 ERC-20 代币接口兼容。Stylus 作为智能合约，也可以实现 ERC-20 接口，确保合约与各种平台和钱包兼容，便于无缝的代币转账和集成。以下是如何通过遵循已建立的 ERC-20 接口与 Stylus 中的 ERC-20 代币进行交互。

# Arbitrum 的 ERC20 实现

让我们看一下这个仓库 [https://github.com/OffchainLabs/stylus-erc20](https://github.com/OffchainLabs/stylus-erc20)

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/stylus_erc20.png)  

## 核心概念

ERC-20 最常见的概念之一是它的属性。每个 ERC-20 代币都需要一个名称、符号和小数位数——就像货币需要一个名称（例如“Ethereum”）、一个交易符号（ETH）和单位分解（18 位小数）。在 Stylus 中，我们使用一个 trait 来确保这些值在**编译时固定**，与 Solidity 的基于存储的方式相比，减少了 gas 成本。  

```rust
pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}
```

继续深入，另一个重要的属性是 `balances`，它跟踪代币所有权，以及 `allowances`，它允许像 DEXs 这样的智能合约代表用户管理代币，启用 `approve` 和 `transferFrom` 等功能。  

```rust
sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
    }
}
```

### ERC20 功能

```solidity
interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address recipient, uint256 amount)
        external
        returns (bool);
    function allowance(address owner, address spender)
        external
        view
        returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address sender, address recipient, uint256 amount)
        external
        returns (bool);
}
```

Stylus 遵循相同的逻辑，但采用 Rust 的**显式错误处理**以保证安全。这可以防止溢出和编译时未预期的状态修改。  

```rust
pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error>;
pub fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Erc20Error>;
```

```rust
pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error>;
pub fn burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error>;
```

### 资源

- 🔗 **[ERC-20 概述 (以太坊)](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)** – 一个面向初学者的 ERC-20 代币介绍，讲解其在以太坊中的角色。  
- 📜 **[EIP-20 规范](https://eips.ethereum.org/EIPS/eip-20)** – 定义 ERC-20 标准的官方以太坊改进提案 (EIP)。  
- 💡 **[OpenZeppelin ERC-20 实现](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol)** – 一个广泛使用且经过战斗测试的 Solidity 实现 ERC-20 代币合约。  