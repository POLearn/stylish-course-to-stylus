## Stylus Equivlant to Solidity Variables

Before we go look at how we can use and deploy a stylus contract, let start simple and look into globals variables in Solidity. Such as `block` and `msg` etc. These are very useful and there is a equilvanet in the STylus SDK. 

An example an important `msg.sender()` in Solidity, In Stylus that,

```rust
use stylus_sdk::{msg};

let address = msg::sender();
```

## Stylus Equivalent of Solidity’s msg

`use stylus_sdk::{msg};`

- msg::reentrant() → Checks if the current call is reentrant
- msg::sender() → Retrieves the address of the account that called the contract (equivalent to Solidity's msg.sender)
- msg::value() → Returns the ETH value (in wei) sent to the contract (equivalent to Solidity's msg.value)

## Stylus Equivalent of Solidity’s block

`use stylus_sdk::{block};`

- block::basefee() → Retrieves the minimum gas fee required for transactions in the current block
- block::chainid() → Returns the unique identifier of the Arbitrum chain the contract is running on
- block::coinbase() → Fetches the address of the entity responsible for posting L1 batches (similar to block.coinbase in Solidity but specific to Arbitrum)
- block::gas_limit() → Gets the maximum gas allowed for transactions within the block
- block::number() → Provides an estimated L1 block number at which the sequencer included the transaction
- block::timestamp() → Returns an estimated Unix timestamp of when the sequencer processed the transaction

## Stylus of keccak

`use stylus_sdk::{crypto};`

- crypto::keccak: efficiently computes the keccak256 hash of the given preimage

These are just some of the global variables available in Stylus for accessing transaction and block data. Stylus provides additional built-in functions for interacting with the VM, such as retrieving gas details, accessing execution context, and handling cryptographic operations. For a complete list of available affordances, check out the official [Stylus VM Affordances documentation](https://docs.arbitrum.io/stylus-by-example/basic_examples/vm_affordances). 🚀

## Loading a smart contract

Let load this contract into an IDE or an a desired enviroment.

This Rust struct, marked with `#[storage]`, defines persistent storage variables for a Stylus smart contract, similar to state variables in Solidity. The `initialized` field (`StorageBool`) tracks whether the contract has been set up, akin to a `bool` in Solidity. The `owner` field (`StorageAddress`) stores the contract owner's address, equivalent to `address` in Solidity. Lastly, `max_supply` (`StorageU256`) holds an unsigned integer value, similar to `uint256` in Solidity. These variables are stored on-chain and persist across transactions, just like Solidity's state variables.

```rust
#[storage]
#[entrypoint]
pub struct Contract {
    initialized: StorageBool,
    owner: StorageAddress,
    max_supply: StorageU256,
}
```

### #[entrypoint]

Another important macro is the `#[entrypoint]`. It's a required macro for Stylus contracts that acts as starting point for Stylus contract execution, ensuring the contract know how and where to compile and **check**. It is typically applied to the main storage struct, making its public methods the primary ones invoked. Only one entrypoint is allowed per contract, preventing multiple top-level execution paths.

### init()

Let’s take a look at the `public fn init` method. This is a common constructor-less method for initializing the contract. It contains a boolean to ensure the initialization process doesn’t run multiple times. What’s more important is how the variables are set. 

To set the `owner` and `max_supply` state variables in the contract, you can use the `msg::sender()` and `block::timestamp()` global variables. To initialize the `owner`, use `self.owner.set(msg::sender())` to assign it to the address of the account that called the contract. Similarly, to set `max_supply`, use `self.max_supply.set(U256::from(block::timestamp()))` to set it to the current block’s timestamp.

These global variables provide essential context within the contract, such as the caller’s address and the block’s timestamp, mimicking similar functionality found in Solidity.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_set.png)

Now that ready. Let compile

> ❗Please note compilation the Stylide IDE, may take upto a minute. Please wait patiently as the contract is being called and checked.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_compile.png)

### Activating Contract

If you get an Metamask, then that completely fine. In Stylus there is a thing called Acivate Program is is done on most contract to improve deployment efficient and make the Stylus contract useable.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_deploy.png)

### Quest: Submit the Deployment

Found the deployment transaction which can typically be found in Metamask or searching your contract on an Explorer. Take that transaction on submit on Proof of Learn. Conguratlation, you have successfully deployed a Stylus Contract! 

You can see that there are methods available that we dicussed such as the `init()` that we'll be calling in the upcoming content.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/variable_deployed.png)
