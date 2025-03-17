# Arrays in Stylus  

Arrays, like in Solidity, can be **fixed-size** (determined at compile time) or **dynamic-size** (growing or shrinking at runtime). In Stylus, arrays follow Rust’s storage principles while maintaining Solidity-like usability.  
## Arrays in Solidity  

Arrays in Solidity allow for storing multiple elements of the same type, and they can be either **fixed-size** or **dynamic**. Fixed-size arrays have a predetermined length, while dynamic arrays can grow or shrink at runtime.  

Solidity arrays are declared using the `type[]` syntax for dynamic arrays or `type[size]` for fixed-size arrays. Arrays can hold any valid Solidity data type, including primitive types (`uint`, `address`, `bool`), structs, or even other arrays.  

```solidity
// Solidity array examples

// Dynamic array (size can change)
uint256[] public dynamicArray;

// Fixed-size array (size is 5)
uint256[5] public fixedArray;
```  

## Arrays in Stylus  

In Stylus, arrays follow a similar concept but are implemented using Rust’s syntax. Stylus provides `StorageVec<T>` for **dynamic arrays**, and fixed-size arrays are declared using `[T; N]`. Both types leverage Rust’s efficient memory management while maintaining Solidity-like behavior.  

```rust
// Stylus Rust array examples
#[storage]
pub struct Contract {
    uint256[3] arr2;
}
```  

### Differences & Similarities  

| Feature       | Solidity                    | Stylus (Rust)               |
|--------------|----------------------------|-----------------------------|
| Syntax       | `type[]` or `type[size]`     | `StorageVec<T>` or `[T; N]` |
| Size         | Fixed or dynamic            | Fixed or dynamic            |
| Iterability  | Supports loops              | Supports loops              |
| Storage Type | Stored in contract storage  | Uses `StorageVec` for dynamic and `[T; N]` for fixed |

### Mapping contract

In this section, we'll dive into a simple array functionality contract. You can get the full source code here and load it in an IDE 
👉 [GitHub Repository](https://github.com/POLearn/stylish-course-to-stylus/tree/master/contract/array)  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_contract.png)

As for the contract states, we have a **dynamic array** `arr`. Unlike fixed-size arrays, dynamic arrays can grow or shrink in size, making them flexible for storing multiple values.

```rust
sol_storage! {
    #[entrypoint]
    pub struct Contract {
        uint256[] arr;
    }
}
```

We can put this into practice by using its built-in methods to efficiently manage and manipulate the stored data.

```rust
// Push an element to the dynamic array
pub fn push(&mut self, i: U256) {
    self.arr.push(i);
}

// Get the element at the index
pub fn get_element(&self, index: U256) -> U256 {
    self.arr.get(index).unwrap()
}

// Get the length of the array
pub fn get_arr_length(&self) -> U256 {
    U256::from(self.arr.len())
}

// Remove an element (does not change the array length)
pub fn remove(&mut self, index: U256) {
    let mut last_element = self.arr.setter(index).unwrap();
    last_element.erase()
}
```

- **`push`**: This function adds a new element to the dynamic array using `.push(i)`. Since Stylus arrays are resizable, new values can be appended at runtime.  

- **`get_element`**: Retrieves a value at a specific index using `.get(index)`. The `.unwrap()` ensures that a valid value is returned, preventing errors if the index is out of bounds.  

- **`get_arr_length`**: Returns the total number of elements in the array. `.len()` retrieves the length, which is then converted to `U256` to match Solidity-like behavior.  

- **`remove`**: This function removes an element at a specific index using `.setter(index).erase()`. However, unlike Solidity's `pop()`, this does **not** shrink the array size; it simply clears the value at the given index.

### Quest: Compiling and Deploying  

Now, let's **compile** and **deploy** the contract.  

![Compiling](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_compile.png)  

![Deploying](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_deploy.png)  

After deploying, you might need to **activate** the contract, which could trigger a second transaction. Once that's done, you're all set!  

Next, we’ll move on to **setting (or moreover pushing) an element in the contract array**. Also, don’t forget to **submit your transaction to Proof of Learn!** 🚀 

You can also check out practical examples here:  
🔗 [Arbitrum Stylus by Example – Arrays](https://arbitrum-stylus-by-example.vercel.app/basic_examples/arrays)