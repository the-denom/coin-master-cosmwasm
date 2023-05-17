# Coin Master Bindings for CosmWasm
​
This crate provides Coin Master bindings to enable your CosmWasm smart contracts to interact with the Coin Master module.
​
## Installation
​
Add the following to your smart contract's `Cargo.toml`:
​
```toml
[dependencies]
coin-master-cosmwasm = { version = "0.1.0" }
```

## Usage

The intention is for these bindings to be used by a cosmos chains bindings library and not directly by a smart contract.
The chains bindings library should provide a wrapper around the messages and queries provided by this crate, and reexport the types.