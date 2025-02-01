Let's take the [Arbitrum Stylus ERC20 Repo](https://github.com/OffchainLabs/stylus-erc20) earlier and load in to an IDE.

The provided image uses Proof of Learn IDE, https://stylus.solide0x.tech/?url=https://github.com/OffchainLabs/stylus-erc20

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_contract.png)  

### ERC20 ABI

If you noticed, when we finished compiling, The compiler isn't able to provide the corresponding ABI. That is totally fine. If you are using Proof of Learn's IDE, then we is a **tab** that allows you to import the ABI.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_abi.png)  

Let's add this to the input provided,

```json
[ { "inputs": [ { "internalType": "address", "name": "", "type": "address" }, { "internalType": "address", "name": "", "type": "address" }, { "internalType": "uint256", "name": "", "type": "uint256" }, { "internalType": "uint256", "name": "", "type": "uint256" } ], "name": "InsufficientAllowance", "type": "error" }, { "inputs": [ { "internalType": "address", "name": "", "type": "address" }, { "internalType": "uint256", "name": "", "type": "uint256" }, { "internalType": "uint256", "name": "", "type": "uint256" } ], "name": "InsufficientBalance", "type": "error" }, { "inputs": [ { "internalType": "address", "name": "owner", "type": "address" }, { "internalType": "address", "name": "spender", "type": "address" } ], "name": "allowance", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "spender", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "approve", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "owner", "type": "address" } ], "name": "balanceOf", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "burn", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [], "name": "decimals", "outputs": [ { "internalType": "uint8", "name": "", "type": "uint8" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [ { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "mint", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "mintTo", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [], "name": "name", "outputs": [ { "internalType": "string", "name": "", "type": "string" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [], "name": "symbol", "outputs": [ { "internalType": "string", "name": "", "type": "string" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [], "name": "totalSupply", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "transfer", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "from", "type": "address" }, { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "transferFrom", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" } ]
```

The above is a classic ABI of the ERC20, that you might see in Solidity based Tokens.

### Quest: Deploying to EDU Chain Testnet

Let's add a **Twist**!

**EDU Chain** operates on Arbitrum Orbit, combining Arbitrum's renowned scalability and rapid transaction speeds with the robust security of the Ethereum network. This unique setup allows EDU Chain to leverage Arbitrum’s features, including compatibility with Stylus. With these powerful capabilities, EDU Chain offers a seamless environment for deploying smart contracts. Now, let’s deploy our ERC20 token on EDU Chain and take full advantage of this cutting-edge infrastructure!

| **Field**            | **Details**                                  |  
|----------------------|----------------------------------------------|  
| **Network Name**      | EDU Chain Testnet                            |  
| **New RPC URL**       | `https://open-campus-codex-sepolia.drpc.org` |  
| **Chain ID**          | `656476`                                     |  
| **Currency Symbol**   | `EDU`                                        |  
| **Block Explorer URL** | [EDU Chain Block Explorer](https://edu-chain-testnet.blockscout.com/) |  

Before deploying, you'll need test tokens. Claim some **$EDU** from the [EDU Chain Faucet](https://educhain-community-faucet.vercel.app/).  

Once connected and funded, you’re ready to deploy your contracts just like in previous sections

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_deployed.png)  

Don't forget to submit your transaction to Proof of Learn and showcase your progress!