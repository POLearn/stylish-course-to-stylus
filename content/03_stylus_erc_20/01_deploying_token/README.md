让我们拿之前的 [Arbitrum Stylus ERC20 Repo](https://github.com/OffchainLabs/stylus-erc20) 加载到一个 IDE 中。

提供的图像使用的是 Proof of Learn IDE，链接是 https://stylus.solide0x.tech/?url=https://github.com/OffchainLabs/stylus-erc20

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_contract.png)

在我们开始查看合约之前，请确保在 `rust-toolchain.toml` 文件中将频道设置为 `1.81.0`

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_toolchain.png)

### ERC20 ABI

如果你注意到，在我们完成编译后，编译器没有提供相应的 ABI。没关系。如果你使用的是 Proof of Learn 的 IDE，那么有一个 **标签页** 允许你导入 ABI。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_abi.png)

让我们将它添加到提供的输入中，

```json
[ { "inputs": [ { "internalType": "address", "name": "", "type": "address" }, { "internalType": "address", "name": "", "type": "address" }, { "internalType": "uint256", "name": "", "type": "uint256" }, { "internalType": "uint256", "name": "", "type": "uint256" } ], "name": "InsufficientAllowance", "type": "error" }, { "inputs": [ { "internalType": "address", "name": "", "type": "address" }, { "internalType": "uint256", "name": "", "type": "uint256" }, { "internalType": "uint256", "name": "", "type": "uint256" } ], "name": "InsufficientBalance", "type": "error" }, { "inputs": [ { "internalType": "address", "name": "owner", "type": "address" }, { "internalType": "address", "name": "spender", "type": "address" } ], "name": "allowance", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "spender", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "approve", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "owner", "type": "address" } ], "name": "balanceOf", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "burn", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [], "name": "decimals", "outputs": [ { "internalType": "uint8", "name": "", "type": "uint8" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [ { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "mint", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "mintTo", "outputs": [], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [], "name": "name", "outputs": [ { "internalType": "string", "name": "", "type": "string" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [], "name": "symbol", "outputs": [ { "internalType": "string", "name": "", "type": "string" } ], "stateMutability": "pure", "type": "function" }, { "inputs": [], "name": "totalSupply", "outputs": [ { "internalType": "uint256", "name": "", "type": "uint256" } ], "stateMutability": "view", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "transfer", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" }, { "inputs": [ { "internalType": "address", "name": "from", "type": "address" }, { "internalType": "address", "name": "to", "type": "address" }, { "internalType": "uint256", "name": "value", "type": "uint256" } ], "name": "transferFrom", "outputs": [ { "internalType": "bool", "name": "", "type": "bool" } ], "stateMutability": "nonpayable", "type": "function" } ]
```

以上是 ERC20 的经典 ABI，你可能会在基于 Solidity 的代币中看到。

### 任务：部署到 EDU 链测试网

让我们来加点 **新意**！

**EDU 链** 基于 Arbitrum Orbit，结合了 Arbitrum 的著名可扩展性和快速交易速度，同时具备了以太坊网络的强大安全性。这一独特的设置使得 EDU 链能够利用 Arbitrum 的特性，包括与 Stylus 的兼容性。借助这些强大的功能，EDU 链为部署智能合约提供了无缝的环境。现在，让我们在 EDU 链上部署我们的 ERC20 代币，充分利用这项尖端基础设施！

| **字段**            | **详细信息**                                  |  
|----------------------|----------------------------------------------|  
| **网络名称**          | EDU 链测试网                                  |  
| **新的 RPC URL**      | `https://open-campus-codex-sepolia.drpc.org` |  
| **链 ID**            | `656476`                                     |  
| **货币符号**         | `EDU`                                        |  
| **区块浏览器 URL**    | [EDU 链区块浏览器](https://edu-chain-testnet.blockscout.com/) |  

在部署之前，您需要一些测试代币。从 [EDU 链水龙头](https://educhain-community-faucet.vercel.app/) 领取一些 **$EDU**。

连接并充值后，就可以像之前一样部署您的合约了。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/token_deployed.png)

别忘了提交您的交易到 Proof of Learn，并展示您的进展！