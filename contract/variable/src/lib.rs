// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use stylus_sdk::alloy_primitives::{U256, Address};
use stylus_sdk::prelude::*;
use stylus_sdk::storage::{StorageAddress, StorageBool, StorageU256};
use stylus_sdk::{block, msg};

#[storage]
#[entrypoint]
pub struct Contract {
    initialized: StorageBool,
    owner: StorageAddress,
    max_supply: StorageU256,
}

#[public]
impl Contract {
    pub fn init(&mut self) -> Result<(), Vec<u8>> {
        let initialized = self.initialized.get();
        if initialized {
            return Ok(());
        }
        self.initialized.set(true);

        self.owner.set("");
        self.max_supply.set(U256::from(""));

        Ok(())
    }

    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    pub fn max(&self) -> U256 {
        self.max_supply.get()
    }
}