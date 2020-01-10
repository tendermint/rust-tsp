# Rust ABCI

Tendermint ABCI server, written in the Rust programming language.

[![Current Version](https://meritbadge.herokuapp.com/abci)](https://crates.io/crates/abci)
[![](https://tokei.rs/b1/github/tendermint/rust-abci)](https://github.com/tendermint/rust-abci)
[![CircleCI](https://circleci.com/gh/tendermint/rust-abci/tree/master.svg?style=shield)](https://circleci.com/gh/tendermint/rust-abci/tree/master)

This library implements the [ABCI
protocol](https://tendermint.com/docs/spec/abci/) and can be used to write ABCI
applications for [Tendermint](https://github.com/tendermint/tendermint/).

## Supported Version

- Tendermint 0.33.0
- ABCI 0.16.1

## Usage

Add `abci` in your `Cargo.toml`'s `dependencies` section:

```toml
[dependencies]
abci = "0.7"
```

Each ABCI application has to implement three core traits corresponding to all three ABCI connections, `Consensus`,
`Mempool` and `Info`.

> Note: Implementations of these traits are expected to be `Send + Sync` and methods take immutable reference of `self`.
So, internal mutability must be handled using thread safe (`Arc`, `Mutex`, etc.) constructs.

After implementing all three above mentioned `trait`s, you can create a `Server` object and use `Server::run()` to start
ABCI application.

`Server::run()` is an `async` function and returns a `Future`. So, you'll need an executor to drive `Future` returned
from `Server::run()`. `async-std` and `tokio` are two popular options. In `counter` example, we use `tokio`'s executor.

To know more, go to `examples/` to see a sample ABCI application.

### Features

- `tokio`: Enables `tokio` backend for running ABCI TCP/UDS server
  - **Enabled** by default.
- `async-std`: Enables `async-std` backend for running ABCI TCP/UDS server
  - **Disabled** by default.

> Features `tokio` and `async-std` are mutually exclusive, i.e., only one of them can be enabled at a time. Compilation
will fail if either both of them are enabled or none of them are enabled.

### Development

This crate already contains the compiled ABCI protobuf messages. If you want to update protobuf messages to a newer version of Tendermint. Run `make update-proto`

## Running the examples

### Tendermint

To run either of the example apps you have to have Tendermint installed and initialised (Remember to run `tendermint init`!). Please install it according to these [instructions](https://docs.tendermint.com/master/introduction/install.html). After initializing and configuring the node, Tendermint can be run with:

```
tendermint node
```

After the node is online, you can run the `counter` example using `cargo run --example counter`.

```
curl localhost:26657/broadcast_tx_commit?tx=0x01
curl localhost:26657/broadcast_tx_commit?tx=0x02
```

For a real life example of an ABCI application you can checkout [Cosmos SDK](https://github.com/cosmos/cosmos-sdk) or [Ethermint](https://github.com/cosmos/ethermint).

#### Tendermint Compatibility Table

| Tendermint | Rust-abci |
| ---------- | :-------: |
| 0.33.0     |   0.7.0   |
| 0.32.7     |   0.6.4   |
| 0.31.7     |   0.5.4   |

## Documentation

Coming soon!

## Join the Community

Find us through a variety of channels [here](https://cosmos.network/community).

### Code of Conduct

Please read, understand and adhere to our [code of conduct](./CODE_OF_CONDUCT.md).

## Credits

- [Jackson Lewis](https://github.com/InquisitivePenguin)
- [Dave Bryson](https://github.com/davebryson)

Original `rust-tsp` made by [Adrian Brink](https://github.com/adrianbrink).
