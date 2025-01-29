# Understanding Storage in Stylus

Storage in Stylus Rust SDK works similarly to Solidity, allowing smart contracts to persist data across transactions. However, because Rust wasn’t originally built for blockchain, Stylus introduces specific storage types to handle on-chain data efficiently.  

## #[storage] macro

The `#[storage]` macro marks a Rust struct as persistent storage. Each field inside the struct must use a specialized storage type that implements the `StorageType` trait.  These storage types ensure that data is efficiently stored and retrieved from the blockchain.

   ```rust
   #[storage]
   pub struct Contract {
       owner: StorageAddress,  // Stores an Ethereum address
       active: StorageBool,    // Stores a boolean value
       sub_struct: SubStruct,  // Nested struct with storage
   }

   #[storage]
   pub struct SubStruct {
       // Additional fields that use StorageType
   }
   ```  

You can found these types in `use stylus_sdk::storage` including types such as ,
```rust
use stylus_sdk::storage::{
    StorageAddress, // Stores Ethereum addresses
    StorageBool,    // Stores boolean values
    StorageU256,    // Stores unsigned 256-bit integers (equivalent to uint256 in Solidity)
    StorageString,  // Stores Solidity-like strings
    StorageMap,     // Stores key-value mappings, similar to Solidity's mapping()
    StorageVec      // Stores dynamic arrays, like Solidity's array types
};
```  

## sol_storage!

Similarly if you prefer Solidity’s familiar syntax, you can define storage using `sol_storage!`, which automatically converts Solidity-style definitions into Rust equivalents.  These type can be imported, `use stylus_sdk::{prelude::*}`

   ```rust
   sol_storage! {
       pub struct Contract {
           address owner;      // Becomes StorageAddress
           bool active;        // Becomes StorageBool
           SubStruct sub_struct;
       }

       pub struct SubStruct {
           mapping(address => uint) balances;  // Becomes StorageMap
           Delegate delegates[];               // Becomes StorageVec
       }
   }
   ```  

This approach makes it easier to migrate Solidity contracts to Rust while preserving the exact storage layout. Since the storage mapping is identical, upgrading from Solidity to Stylus Rust is seamless—just copy and paste your type definitions!

To learn more about storage in the Stylus Rust SDK, check out the official documentation:  
👉 [Stylus Rust SDK Storage Guide](https://docs.arbitrum.io/stylus/reference/rust-sdk-guide#storage)  