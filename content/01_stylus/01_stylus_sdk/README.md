# Stylus SDK

The Stylus SDK is a powerful toolkit designed for developing smart contracts using Rust and other WASM-compatible languages. Built on top of Alloy, a Rust library for Ethereum-based development, the Stylus SDK empowers developers to create high-performance decentralized applications (dApps) on Arbitrum with ease. It integrates seamlessly with existing Rust libraries and offers a set of features that enable efficient contract development, deployment, and interaction.

For developers coming from the Solidity ecosystem, the ability to use familiar syntax is a huge advantage. It allows them to quickly understand and work with Rust, reducing the time needed to learn a completely new language. Moreover, Stylus SDK share similar programming concepts with Solidity, allowing developers to leverage their knowledge of Rust and Solidity when working with the Stylus SDK.

## Solidity Smart Contract

If you know Solidity, this is a basic contract that manages ownership through an address. It includes two main functions: one to retrieve the current owner’s address and another to update the owner by setting a new address. The contract uses a public state variable to store the owner’s address, allowing other parts of the system to interact with or modify the ownership
```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

contract Ownable {
    address public owner;

    function owner() public pure returns(address) {
        return owner;
    }

    function setOwner(address _owner) public {
        owner = _owner;
    }
}
```

## Stylus Rust SDK Smart Contract

The **Ownable** smart contract in Stylus SDK closely mirrors the Solidity version, maintaining the core logic of managing ownership through an address. In this Stylus contract, the `owner` is stored using the `StorageAddress` type, a special type in Stylus that persists the owner's address on-chain. The `owner()` function retrieves the current owner’s address, similar to the Solidity contract's getter function, while the `set_owner()` function allows updating the owner's address, akin to the `setOwner()` function in Solidity.

```rust
use stylus_sdk::alloy_primitives::{Address};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress6};

#[storage]
#[entrypoint]
pub struct Ownable {
    owner: StorageAddress,
}

#[external]
impl Ownable {
    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    pub fn set_owner(&self, _owner: StorageAddress) {
        self.owner.set(_owner);
    }
}
```

The key differences lie in how Stylus uses Rust syntax and the `#[storage]` attribute, which marks the `owner` as a storage variable, and `#[entrypoint]`, which specifies the contract's entry point. Despite these differences, the fundamental ownership management logic remains almost identical, demonstrating how Stylus allows developers to write smart contracts in Rust while keeping familiar patterns from Solidity intact, making it easy for Solidity developers to transition to Rust-based smart contract development.