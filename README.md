# Rust ABCI

Tendermint ABCI server, written in the Rust programming language.

[![Current Version](https://meritbadge.herokuapp.com/abci)](https://crates.io/crates/abci)
[![](https://tokei.rs/b1/github/tendermint/rust-abci)](https://github.com/tendermint/rust-abci)
[![CircleCI](https://circleci.com/gh/tendermint/rust-abci/tree/master.svg?style=shield)](https://circleci.com/gh/tendermint/rust-abci/tree/master)

This library implements the [ABCI
protocol](https://tendermint.com/docs/spec/abci/) and can be used to write ABCI
applications for [Tendermint](https://github.com/tendermint/tendermint/).

## Supported Version

- Tendermint 0.32.0
- ABCI 0.16.0

## Installation

### Dependencies

Make sure that you have Rust and Cargo installed. The easiest way is to follow the instructions on [rustup](https://rustup.rs/).

To test the examples, please clone this repository.

```
git clone https://github.com/tendermint/rust-abci.git
```

The `empty_app` example, found under the `examples` folder, is a good demonstration/bare minimum foundation for a Rust ABCI app.

To use this library to build your own ABCI apps in Rust you have to include the following in your `Cargo.toml` file.

```toml
[dependencies]
abci = "0.6.2"
```

### Development

This crate already contains the compiled ABCI protobuf messages. If you want to update protobuf messages to a newer version of Tendermint. Run `make update-proto`

## Running the examples

### Tendermint

To run either of the example apps you have to have Tendermint installed and initialised (Remember to run `tendermint init`!). Please install it according to these [instructions](https://tendermint.com/docs/introduction/install.html). After initializing and configuring the node, Tendermint can be run with:

```
tendermint node
```

After the node is online, you can run the `empty_app` example using `cargo run --example empty_app`.

To run the `counter_app` run `cargo run --example counter_app` and send transaction to Tendermint via:

```
curl localhost:26657/broadcast_tx_commit?tx=0x01
curl localhost:26657/broadcast_tx_commit?tx=0x02
```

For a real life example of an ABCI application you can checkout [Cosmos SDK](https://github.com/cosmos/cosmos-sdk) or [Ethermint](https://github.com/cosmos/ethermint).

#### Tendermint Compatibility Table

| Tendermint | Rust-abci |
| ---------- | :-------: |
| 0.32.7     |   0.6.3   |
| 0.32.2     |   0.6.2   |
| 0.31.7     |   0.5.4   |

## Documentation

Coming soon!

## Join the Community

Find us through a variety of channels [here](https://cosmos.network/community).

### Code of Conduct

Please read, understand and adhere to our [code of conduct](./CODE_OF_CONDUCT.md).

## Credits

- [Jackson Lewis](https://github.com/JacksonCoder)
- [Dave Bryson](https://github.com/davebryson)

Original `rust-tsp` made by [Adrian Brink](https://github.com/adrianbrink).
