# Setting setMyNestedMap

Now that we have our contract deployed, it's time to **set values** in our nested mapping using the `setMyNestedMap` function. This function allows us to update specific entries within the `my_nested_map`.

Let's take a quick recap.

```rust
pub fn set_my_nested_map(&mut self, index: U256, target: Address, new_value: bool) {
    self.my_nested_map.setter(index).setter(target).set(new_value);
}
```

- **`index`**: This is the primary key (`U256`) that identifies the outer mapping.
- **`target`**: The address (`Address`) of the account you're interacting with.
- **`new_value`**: The new boolean value you wish to set.

The `setter()` function is key to navigating through the nested structure. First, we use `setter(index)` to access the nested mapping associated with the given `index`. Then, we call `setter(target)` to reach the specific address. Finally, we use `.set(new_value)` to update the value.

### Quest: Call the `setMyNestedMap`

To complete this quest, call the `setMyNestedMap` method with the following values: set the **`index`** to `10` (a `U256` key), provide any valid **`target`** address (use your own address), and set the **`new_value`** to `true` (a boolean value).

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_set.png)  

You can test the value by calling `getMyNestedMap`.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/mapping_value.png)

Finally, submit the transaction to Proof of Learn to complete the quest. This will help you interact with nested mappings and modify contract state.