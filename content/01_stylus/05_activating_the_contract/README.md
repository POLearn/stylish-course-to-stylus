# 激活 Stylus 合约

## 概述

Stylus 合约从高级代码（Rust、C、C++）开始，编译为 WebAssembly（WASM），然后发布到链上。然而，WASM 必须在执行之前 **激活**。

## 激活过程

1. **编译**：合约使用 Stylus SDK（Rust）或 Clang（C、C++）进行编译。
2. **ArbWasm 预编译**：WASM 被转换为优化后的本地机器代码（x86/ARM），以便执行。
3. **安全与验证**：中间件检查通过 gas 测量、深度检查和内存计费确保安全。
4. **执行**：激活后，合约可以完全执行。

## Ink：Stylus 中的 Gas 测量

Stylus 引入了 **ink**，一种比以太坊 gas 小得多的 gas 单位。WASM 执行速度快于 EVM，使 ink 更加精确。ink 与 gas 的比率可以根据未来硬件的改进进行调整。

对于一些合约，如果具有相似字节码的合约已经被激活，则此步骤可能是可选的。

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract.png)