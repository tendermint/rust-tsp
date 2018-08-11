# Rust TSP

A rust implementation of the ABCI protocol for Tendermint.

[![](https://tokei.rs/b1/github/tendermint/rust-tsp)](https://github.com/tendermint/rust-tsp)

## About

This library implements the ABCI protocol and can be used to write ABCI applications for Tendermint in rust.
Here you can find more information about [Tendermint](https://github.com/tendermint/tendermint) and [ABCI application](https://github.com/tendermint/abci).

For a real life example of an ABCI application you can checkout [Ethermint](https://github.com/cosmos/ethermint) or [Cosmos SDK](https://github.com/cosmos/cosmos-sdk).

## Installation

### Dependencies

Make sure that you have Rust and Cargo installed. The easiest way is to follow the instructions on [rustup](https://rustup.rs/).

To test the examples, please clone this repository.

```bash
git clone git@github.com:tendermint/rust-tsp.git
```

The `empty_app` example, found under the `examples` folder, is a good demonstration/bare minimum foundation for a Rust ABCI app.

To use this library to build your own ABCI apps in rust you have to include the following in your `Cargo.toml` file.

```toml
[dependencies]
abci = "0.3.0"
```

## Running the examples

### Tendermint

To run either of the example apps you have to have Tendermint installed and initialised (Remember to run `tendermint init`!). Please install it according to these [instructions](https://github.com/tendermint/tendermint). After initializing and configuring the node, Tendermint can be run with:

```bash
tendermint node
```

If you wish to not create new blocks each second, make sure to add the `--consensus.create_empty_blocks=0` flag.

After the node is online, you can run the `empty_app` example using `cargo run --example empty_app`.

## Documentation

Coming soon!

## Join the Community

Find us through a variety of channels [here](https://cosmos.network/community).

### Code of Conduct

Please read, understand and adhere to our [code of conduct](https://github.com/tendermint/rust-tsp/blob/master/CODE_OF_CONDUCT.md).

## Credits

Original `rust-tsp` made by Adrian Brink. New `abci` package and fixes based on `rust-tsp` made by Jackson Lewis.
