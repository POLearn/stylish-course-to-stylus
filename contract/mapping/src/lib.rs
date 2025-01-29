// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{prelude::*, alloy_primitives::{Address, U256}};
use stylus_sdk::storage::{*};

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        mapping(address => bool) my_map;
        mapping(uint256 => mapping(address => bool)) my_nested_map;
    }
}

#[public]
impl Contract {
    // Mapping always returns a value.
    // If the value was never set, it will return the default value.
    pub fn get_my_map(&self, target: Address) -> bool {
        self.my_map.get(target)
    }

    // Update the value at this address
    pub fn set_my_map(&mut self, target: Address, new_value: bool) {
        self.my_map.setter(target).set(new_value);
    }

    // Reset the value to the default value.
    pub fn remove_my_map(&mut self, target: Address) {
        self.my_map.delete(target);
    }

    // Mapping always returns a value.
    // If the value was never set, it will return the default value.
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
}