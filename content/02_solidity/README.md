Now that we've deployed a simple contract and understood the structure and basics of Rust-based Stylus contracts, let's dive deeper into more complex types used in Solidity that can also be applied in Stylus.

## Mappings

In Stylus, mappings are similar to Solidity's hash tables, allowing you to store data as key-value pairs. The key can be any built-in type, and the value can be any type of data you need to store. 

In Rust, mappings are created with `StorageMap<keyType, StorageType>`, where `keyType` is typically from `alloy_primitives` and `StorageType` is any type that fits your needs (such as `StorageU256`, `StorageAddress`, etc.). 

In Solidity, mappings are declared with the syntax `mapping(keyType => valueType)`, where `keyType` can be basic types like `uint`, `address`, `string`, or even another contract, and `valueType` can be any type, including arrays or other mappings.

One thing to remember: **Mappings are not iterable**. This means you cannot loop through them directly, and you must have a reference to each key to access its corresponding value.

## Arrays

Arrays in Stylus work similarly to Solidity, but there are some key differences when it comes to how they're declared and used.

1. **Dynamic-Size Arrays:** These arrays can change size at runtime, meaning elements can be added or removed as needed. They are flexible and widely used when the size of the data structure is unknown beforehand.

2. **Fixed-Size Arrays:** These arrays have a size that is determined at compile time. Once the size is set, it cannot be changed, which can be useful when you know the exact number of elements you need to store.

3. **Custom Struct Element Arrays:** Arrays can also store more complex data types like structs. In this case, each element in the array is a custom struct, allowing for highly organized data storage.

Arrays in Stylus provide the same versatility as in Solidity, giving you the freedom to choose between dynamic or fixed sizes depending on your contract's needs.