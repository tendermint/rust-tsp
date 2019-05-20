//! # Tendermint ABCI library for Rust
//!
//! This library provides an application Trait and TCP server for implementing Tendemint ABCI
//! application in Rust.  The Application Trait provides default implementations for each callback
//! to simplify development.
//!
//! ## Example
//!
//! Here's a simple example that communicates with Tendermint. Defaults callbacks are handled by
//! the Trait.  The app doesn't do any actual processing on a transaction.
//!
//! ```rust,no_run
//! #[derive(Clone, Copy)]
//! struct EmptyApp;
//!
//! impl abci::Application for EmptyApp {}
//!
//! fn main() {
//!     abci::run_local(EmptyApp);
//! }
//!```
//!
extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate integer_encoding;
extern crate mockstream;
extern crate protobuf;
extern crate tokio;

use std::net::SocketAddr;

pub use messages::abci::*;
pub use messages::merkle::*;
pub use messages::types::*;
use server::serve;

mod codec;
mod messages;
mod server;
mod stream;

/// Main Trait for an ABCI application. Provides generic responses for all callbacks
/// Override desired callbacks as needed.  Tendermint makes 3 TCP connections to the
/// application and does so in a synchonized manner.
pub trait Application {
    /// Query Connection: Called on startup from Tendermint.  The application should normally
    /// return the last know state so Tendermint can determine if it needs to replay blocks
    /// to the application.
    fn info(&mut self, _req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }

    /// Query Connection: Set options on the application (rarely used)
    fn set_option(&mut self, _req: &RequestSetOption) -> ResponseSetOption {
        ResponseSetOption::new()
    }

    /// Query Connection: Query your application. This usually resolves through a merkle tree holding
    /// the state of the app.
    fn query(&mut self, _req: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }

    /// Mempool Connection:  Used to validate incoming transactions.  If the application reponds
    /// with a non-zero value, the transaction is added to Tendermint's mempool for processing
    /// on the deliver_tx call below.
    fn check_tx(&mut self, _req: &RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    /// Consensus Connection:  Called once on startup. Usually used to establish initial (genesis)
    /// state.
    fn init_chain(&mut self, _req: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    /// Consensus Connection: Called at the start of processing a block of transactions
    /// The flow is:
    /// begin_block()
    ///   deliver_tx()  for each transaction in the block
    /// end_block()
    /// commit()
    fn begin_block(&mut self, _req: &RequestBeginBlock) -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    /// Consensus Connection: Actually processing the transaction, performing some form of a
    /// state transistion.
    fn deliver_tx(&mut self, _p: &RequestDeliverTx) -> ResponseDeliverTx {
        ResponseDeliverTx::new()
    }

    /// Consensus Connection: Called at the end of the block.  Often used to update the validator set.
    fn end_block(&mut self, _req: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    /// Consensus Connection: Commit the block with the latest state from the application.
    fn commit(&mut self, _req: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }
}

/// Setup the app and start the server using localhost and default tendermint port 26658
pub fn run_local<A>(app: A)
where
    A: Application + 'static + Send + Sync + Copy,
{
    let addr = "127.0.0.1:26658".parse().unwrap();
    run(addr, app);
}

/// Setup the application and start the server. Use this fn when setting different ip:port.
pub fn run<A>(listen_addr: SocketAddr, app: A)
where
    A: Application + 'static + Send + Sync + Copy,
{
    serve(app, listen_addr).unwrap();
}
