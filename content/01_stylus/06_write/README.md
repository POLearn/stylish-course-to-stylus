# Calling a smart contract method

In this guide, we’ll walk you through how to call the `init` method, which is a write method in a contract. A write method is a function that modifies or sets the state of the contract, such as changing variables, updating values, or performing actions that change the blockchain's state.

## init

The `init` method is typically used to initialize the contract's state. In the case of the contract example, it sets the `owner` and `max_supply` variables. It is often used as a one-time setup function that only runs once when the contract is first deployed or when it is called for the first time.

## Quest: Calling the `init` Method

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract_init.png)  

With the smart contract deployed, you can call the `init` method. This method is a write operation, meaning it will modify the state of the contract. You typically call the `init` method through a transaction that is broadcast to the network.

![](https://raw.githubusercontent.com/POLearn/stylish-course-to-stylus/refs/heads/master/content/assets/images/contract_init_success.png)  

Now you can call the other methods such as `owner` and `max` to see the values. We will go through calling a smart contract in the next section.