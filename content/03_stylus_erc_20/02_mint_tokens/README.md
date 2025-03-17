# Quest: Mint Your Token! 🚀

Now that your Stylus ERC20 contract is deployed, you can interact with it just like any ERC20 token on Ethereum or Base. But here's the twist—this token is built using Rust and runs on an EVM-compatible chain like EDU Chain. Cool, right? 🎉 

In this section, we'll walk you through minting a simple `1000000` tokens and sending them to your own address. Let's break down how this works. The code below is the `mint` function in the deployed contract which allows us to create new tokens and assign them to an address. The function takes a `value` parameter (in our case, `1000000` tokens) and calls the ERC20 `mint` method we defined earlier. It uses `msg::sender()` to grab the address that initiated the minting and assigns the tokens to it. In simple terms, this function enables us to generate fresh tokens and send them to the user who called the function.

```rust
/// Mints tokens
pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
    self.erc20.mint(msg::sender(), value)?;
    Ok(())
}
```

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_mint.png)  

Once you've minted your tokens, don’t forget to submit the transaction to Proof of Learn and show off your progress! 

Minting tokens is the process of creating new tokens and sending them to a specific address. With ERC20 tokens, this operation not only adds tokens to an account but also unlocks all the cool features of ERC20 tokens like transferring, checking balances, and more. To check your token balance, simply call `balanceOf` with the address that minted the tokens (or any address, for that matter)!

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_balanceof.png)  

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_balance.png)
