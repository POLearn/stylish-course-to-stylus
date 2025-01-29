# Stylus smart contract project structure

A typical **Stylus contract** in Rust consists of two main files: `lib.rs` and `main.rs`. Each file has its distinct role, providing the structure and logic that together form the complete contract. Let’s break down how these files work together:

## `lib.rs` – Core Contract Logic

In `lib.rs`, the contract logic is defined. This file contains the main business logic of the smart contract, including the data structures, entry points, and functions that interact with the blockchain. 

If we take a look at the previous Ownable Rust code, we can see this is a typically structure of a contract, containing

- **`#[storage]`**: Marks the struct `Ownable` as a storage structure. It holds the state of the contract (e.g., the `owner` variable in this case).
- **`#[entrypoint]`**: Defines the entry point to the contract, marking the functions that can be called externally.
- **`#[external]`**: The functions under this annotation are exposed to the blockchain. In this example, `owner` and `set_owner` are external functions, with `set_owner` changing the contract's state.

This is where most of the contract’s core logic resides, allowing it to interact with the blockchain and store important state information like the contract owner.

## `main.rs` – Initialization and ABI Export

The `main.rs` file in a Stylus contract typically contains the initialization logic and is crucial for managing how the contract is executed or deployed. It also handles things like exporting the ABI (Application Binary Interface) and other initialization routines.

Here’s an example of how **`main.rs`** is structured:

```rust
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[cfg(not(any(test, feature = "export-abi")))]
#[no_mangle]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
```

- **`#[cfg]`**: Conditional compilation is used here. If the feature `export-abi` is enabled, it will generate the ABI information, making it accessible to users who may need to interact with the contract.
- **`no_main`**: If not exporting ABI or in a testing environment, this prevents the default main function from being compiled into the contract, which is important for minimal contract size.
- **`print_abi`**: This function will print the ABI when the `export-abi` feature is enabled, which helps in generating and verifying the interface of the contract.

While **`lib.rs`** contains the core logic of the contract, **`main.rs`** manages initialization tasks, handles ABI generation, and ensures that the contract is ready for deployment on the blockchain.

## Cargo.toml

Since this is a Rust program as well, an important configuration file is the `Cargo.toml` which is the manifest for a Stylus smart contract, defining essential metadata, dependencies, and build configurations. Note here we have `[dependencies]` section includes key libraries such as `stylus-sdk` for smart contract development, `alloy-primitives` for Ethereum-compatible types, and `mini-alloc` for efficient memory allocation. The `[features]` section enables optional functionalities like ABI export and debugging, which is often included as it'll allow the contract to to easily generate an ABI and smart contract for deployment