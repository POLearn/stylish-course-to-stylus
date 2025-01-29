# Reading from the contract

Just like in Solidity, reading from a contract allows you to get the current values of the contract’s state variables. If you see the other methods of the contract mainly,

```rust
pub fn owner(&self) -> Address {
    self.owner.get()
}

pub fn max(&self) -> U256 {
    self.max_supply.get()
}
```

these value methods can be used as getter methods allowing us to inspect contract state which can be important information like who owns the contract or what the maximum supply of a token is. Remember these values *set* with the `init` method called before.

## Quest: Read with a Twist

For this quest. Let's change it up. Load up the contract.

![](variable_load.png)

Then similarly to the calling the init, you can click on the `max` method to call or **read** the of contract values. In the console you should get a response from the contract, which was the timestamp which that contract was called.

![](variable_load.png)

Now submit the value in double quotes such its `"ANSWER"` to Proof of Learn to earn this quest.
