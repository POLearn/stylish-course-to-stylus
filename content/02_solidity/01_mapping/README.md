https://github.com/polearn/stylish-course-to-stylus/tree/master/contract/mapping
Reference - https://arbitrum-stylus-by-example.vercel.app/basic_examples/mapping


# Mappings  

Mappings are fundamental data structures in Solidity, functioning as hash tables that store key-value pairs. They are commonly used for efficient storage and retrieval of data, such as tracking balances or ownership. Stylus, the Rust-based smart contract framework for Arbitrum, provides a similar concept using `StorageMap`.  

## Mappings in Solidity

In Solidity, mappings are declared using the `mapping(keyType => valueType)` syntax. The key can be any built-in value type (e.g., `uint`, `address`, `bytes`), and the value can be any type, including another mapping or an array. However, mappings in Solidity are not iterable, meaning you cannot loop through them directly.  

```solidity
// Solidity mapping example
mapping(address => uint256) public balances;
```

## Mappings in Stylus

In Stylus, mappings are implemented using `StorageMap<keyType, StorageType>`, which follows Rust’s syntax. The `keyType` must be a type from `alloy_primitives`, while the `valueType` must be a supported `StorageType`. This structure ensures efficient key-value storage in Rust-based smart contracts.  

```rust
// Stylus Rust mapping example
#[storage]
pub struct Contract {
    balances: StorageMap<Address, StorageU256>,
}
```

### Differences & Similarities

| Feature        | Solidity                     | Stylus (Rust)              |
|---------------|-----------------------------|----------------------------|
| Syntax        | `mapping(keyType => valueType)` | `StorageMap<keyType, StorageType>` |
| Key Types     | Built-in types, `bytes`, `string`, contract addresses | Types from `alloy_primitives` |
| Value Types   | Any type, including other mappings or arrays | Any `StorageType` |
| Iterability   | Not iterable                 | Not iterable               |

### Mapping contract

In this section, we'll dive into mappings and nested mappings in Stylus. You can access the full source code here:  
👉 [GitHub Repository](https://github.com/POLearn/stylish-course-to-stylus/tree/master/contract/mapping)  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_contract.png)

```rust
sol_storage! {
    #[entrypoint]
    pub struct Contract {
        mapping(address => bool) my_map;
        mapping(uint256 => mapping(address => bool)) my_nested_map;
    }
}
```  

This is the *entrypoint* of the contract. As you may recall, `sol_storage!` defines the contract’s storage structure, allowing us to create mapping variables similar to Solidity while leveraging Rust’s syntax.  

- **`my_map`**: A simple mapping that associates an `address` with a `bool`, making it useful for tracking permissions, whitelists, or other flags.  
- **`my_nested_map`**: A **nested mapping** where a `uint256` key (such as an ID) maps to another `mapping(address => bool)`. This structure is commonly used for tracking approvals, role-based access, or multi-level permissions.  

Now, let’s explore how to interact with **nested mappings** in Stylus. The following functions allow us to **read**, **update**, and **remove** values within `my_nested_map`.  

```rust
pub fn get_my_nested_map(&self, index: U256, target: Address) -> bool {
    self.my_nested_map.get(index).get(target)
}

// Update the value at this address
pub fn set_my_nested_map(&mut self, index: U256, target: Address, new_value: bool) {
    self.my_nested_map.setter(index).setter(target).set(new_value);
}

// Reset the value to the default value.
pub fn remove_my_nested_map(&mut self, index: U256, target: Address) {
    self.my_nested_map.setter(index).delete(target);
}
```

- **`get_my_nested_map`**: This function retrieves a stored value using the `.get()` method. First, it accesses the mapping at a specific `index` (a `U256` key), then retrieves the boolean value linked to a `target` address. This is useful for checking permissions or tracking approvals.  

- **`set_my_nested_map`**: To update or insert values, the `.setter()` function is used to navigate through the nested mappings before calling `.set(new_value)`. The first `.setter(index)` accesses the mapping at `index`, while the second `.setter(target)` reaches the specific address. Finally, `.set(new_value)` updates the boolean value.  

- **`remove_my_nested_map`**: The `.delete()` function resets a mapping entry, effectively removing stored data. By using `.setter(index).delete(target)`, we clear the mapping for a given `target` at a specific `index`, which is useful for revoking permissions or resetting values.  

Now that you understand how nested mappings work, the `my_map` functionality is a **simpler** version of the same concept. It uses the same methods—`.get()`, `.setter()`, `.set()`, and `.delete()`—to perform **CRUD (Create, Read, Update, Delete)** operations on a **single-layer mapping** instead of a nested one.

### Quest: Compiling and Deploying  

Now, let's **compile** and **deploy** the contract.  

![Compiling](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_compile.png)  

![Deploying](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_deploy.png)  

After deploying, you might need to **activate** the contract, which could trigger a second transaction. Once that's done, you're all set!  

Next, we’ll move on to **setting values in the nested mapping**. Also, don’t forget to **submit your transaction to Proof of Learn!** 🚀  