# Pushing Elements to the Array  

Now that we have our contract deployed, it's time to **add values** to our dynamic array using the `push` function. This function allows us to insert new elements into the `arr` array.  

Let's take a quick recap.  

```rust
pub fn push(&mut self, i: U256) {
    self.arr.push(i);
}
```  

- **`i`**: The `U256` value to be added to the array.  

The `.push()` function dynamically increases the array size by appending the new element at the end. Since `arr` is a **dynamic array**, we can continue adding elements without needing to predefine the size.  

### Quest: Call the `push` function  

To complete this quest, call the `push` method with the value `42161` (a `U256` value).

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_push.png)  

You can verify the insertion by retrieving the element using `get_element` (at index 0) and checking the array length with `get_arr_length` (should be 1).  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/array_get.png)  

Finally, submit the transaction to Proof of Learn to complete the quest. This will help you interact with dynamic arrays and modify contract state.