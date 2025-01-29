// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{prelude::*, alloy_primitives::{U256}};
use stylus_sdk::storage::{*};

sol_storage! {
    #[entrypoint]
    pub struct Contract {
        uint256[] arr;
    }
}

#[public]
impl Contract {
    // dynamic array
    // push an element to the dynamic array
    pub fn push(&mut self, i: U256) {
        self.arr.push(i);
    }

    // get the element at the index
    pub fn get_element(&self, index: U256) -> U256 {
        self.arr.get(index).unwrap()
    }

    // get the length of the array
    pub fn get_arr_length(&self) -> U256 {
        U256::from(self.arr.len())
    }

    // remove will not change the length of the array
    pub fn remove(&mut self, index: U256) {
        let mut last_element = self.arr.setter(index).unwrap();
        last_element.erase()
    }
}