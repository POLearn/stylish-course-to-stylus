# Activating a Stylus Contract

## Overview

Stylus contracts start as high-level code (Rust, C, C++) and compile into WebAssembly (WASM) before being posted on-chain. However, WASM must be **activated** before execution.

## Activation Process

1. **Compilation**: The contract is compiled using the Stylus SDK (Rust) or Clang (C, C++).
2. **ArbWasm Precompile**: WASM is converted into optimized native machine code (x86/ARM) for execution.
3. **Security & Validation**: Middleware checks ensure safety via gas metering, depth checking, and memory charging.
4. **Execution**: After activation, the contract is fully functional.

## Ink: Gas Measurement in Stylus

Stylus introduces **ink**, a gas unit much smaller than Ethereum’s gas. WASM executes faster than the EVM, making ink more precise. The ink-to-gas ratio can adapt based on future hardware improvements.

This may be optional for some, if a contract of similar bytecode is already activated.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract.png)