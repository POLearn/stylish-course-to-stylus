# ERC-20 in Stylus

In Solidity, the **ERC-20** standard defines a common set of rules for creating fungible tokens. These tokens are widely used in decentralized applications (dApps) to represent assets, currencies. ERC-20 Compliance: The contract implements standard functions such as total_supply, balance_of, transfer, approve, and transfer_from, ensuring compatibility with existing ERC-20 token interfaces. STylus as a smart contract can also have its the ERC-20 interface, where contracts can ensure compatibility with various platforms and wallets, enabling seamless token transfers and integrations. Here’s how you can interact with ERC-20 tokens in Stylus by following the established ERC-20 interface.

# Arbitrum's ERC20 implementation

LEt's take a look at the repo [https://github.com/OffchainLabs/stylus-erc20](https://github.com/OffchainLabs/stylus-erc20)

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/stylus_erc20.png)  

## Core Concepts

One of the most common concepts of ERC20 are its properties. Every ERC-20 token needs a name, a symbol, and a decimal count—just like a currency needs a name (e.g., "Ethereum"), a ticker symbol (ETH), and a unit breakdown (18 decimals). In Stylus, we use a trait to ensures these values are **fixed at compile time**, reducing gas costs compared to Solidity’s storage-based approach.  

```rust
pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}
```

If we go futher down, another important properties are the `balances` which track token ownership, and `allowances` which enable functions like approve and transferFrom, allowing smart contracts (like DEXs) to manage tokens on behalf of users. 

```rust
sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
    }
}
```

### ERC20 Functionality

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


Stylus follows the same logic but with Rust’s **explicit error handling** for safety, This prevents overflows and unintended state modifications at compile time.  

```rust
pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error>;
pub fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Erc20Error>;
```

```rust
pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error>;
pub fn burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error>;
```

### Resources

- 🔗 **[ERC-20 Overview (Ethereum)](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)** – A beginner-friendly introduction to ERC-20 tokens and their role in Ethereum.  
- 📜 **[EIP-20 Specification](https://eips.ethereum.org/EIPS/eip-20)** – The official Ethereum Improvement Proposal (EIP) that defines the ERC-20 standard.  
- 💡 **[OpenZeppelin ERC-20 Implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol)** – A widely-used, battle-tested Solidity implementation of ERC-20.  