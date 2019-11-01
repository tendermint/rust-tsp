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
#[macro_use]
extern crate crossbeam_channel;
extern crate env_logger;
extern crate futures;
extern crate integer_encoding;
#[macro_use]
extern crate log;
extern crate core;
extern crate protobuf;
extern crate tokio;

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::net::SocketAddr;
use std::thread;
use tokio::sync::oneshot;

pub use crate::messages::abci::*;
pub use crate::messages::merkle::*;
pub use crate::messages::types::*;
use crate::server::serve;

mod codec;
mod messages;
mod server;

/// Main Trait for an ABCI application. Provides generic responses for all callbacks
/// Override desired callbacks as needed.  Tendermint makes 3 TCP connections to the
/// application and does so in a synchonized manner.
pub trait Application: 'static + Send {
    /// Signal the start of the server to the application, passing along an Exiter.
    fn start(&mut self, _exiter: Exiter) {}

    /// Signal the exit of the server to the application.
    fn exit(&mut self) {}

    /// Query Connection: Called on startup from Tendermint.  The application should normally
    /// return the last know state so Tendermint can determine if it needs to replay blocks
    /// to the application.
    fn info(&mut self, _req: RequestInfo, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseInfo::new();
        response.set_info(res);
        let _ = responder.respond(response);
    }

    /// Query Connection: Set options on the application (rarely used)
    fn set_option(&mut self, _req: RequestSetOption, responder: Responder) {
        let mut response = Response::new();
        let options = ResponseSetOption::new();
        response.set_set_option(options);
        let _ = responder.respond(response);
    }

    /// Query Connection: Query your application. This usually resolves through a merkle tree holding
    /// the state of the app.
    fn query(&mut self, _req: RequestQuery, responder: Responder) {
        let mut response = Response::new();
        let query = ResponseQuery::new();
        response.set_query(query);
        let _ = responder.respond(response);
    }

    /// Mempool Connection:  Used to validate incoming transactions.  If the application reponds
    /// with a non-zero value, the transaction is added to Tendermint's mempool for processing
    /// on the deliver_tx call below.
    fn check_tx(&mut self, _req: RequestCheckTx, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseCheckTx::new();
        response.set_check_tx(res);
        let _ = responder.respond(response);
    }

    /// Consensus Connection:  Called once on startup. Usually used to establish initial (genesis)
    /// state.
    fn init_chain(&mut self, _req: RequestInitChain, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseInitChain::new();
        response.set_init_chain(res);
        let _ = responder.respond(response);
    }

    /// Consensus Connection: Called at the start of processing a block of transactions
    /// The flow is:
    /// begin_block()
    ///   deliver_tx()  for each transaction in the block
    /// end_block()
    /// commit()
    fn begin_block(&mut self, _req: RequestBeginBlock, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseBeginBlock::new();
        response.set_begin_block(res);
        let _ = responder.respond(response);
    }

    /// Consensus Connection: Actually processing the transaction, performing some form of a
    /// state transistion.
    fn deliver_tx(&mut self, _p: RequestDeliverTx, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseDeliverTx::new();
        response.set_deliver_tx(res);
        let _ = responder.respond(response);
    }

    /// Consensus Connection: Called at the end of the block.  Often used to update the validator set.
    fn end_block(&mut self, _req: RequestEndBlock, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseEndBlock::new();
        response.set_end_block(res);
        let _ = responder.respond(response);
    }

    /// Consensus Connection: Commit the block with the latest state from the application.
    fn commit(&mut self, _req: RequestCommit, responder: Responder) {
        let mut response = Response::new();
        let res = ResponseCommit::new();
        response.set_commit(res);
        let _ = responder.respond(response);
    }
}

/// Represents the type of response that can be send with a Responder.
pub enum ResponseType {
    Info,
    SetOption,
    Query,
    CheckTx,
    InitChain,
    BeginBlock,
    DeliverTx,
    EndBlock,
    Commit,
    Flush,
    Echo,
    Exception,
}

/// To be used by the application to provide a response,
/// possibly asynchronously, for a specific request.
pub struct Responder {
    response_sender: Sender<(Response, oneshot::Sender<Response>)>,
    network_sender: oneshot::Sender<Response>,
    response_type: ResponseType,
}

impl Responder {
    pub fn new(
        response_sender: Sender<(Response, oneshot::Sender<Response>)>,
        network_sender: oneshot::Sender<Response>,
        response_type: ResponseType,
    ) -> Self {
        Responder {
            response_sender,
            network_sender: network_sender,
            response_type,
        }
    }

    fn check_response(&self, response: &Response) -> Result<(), ()> {
        let has_expected = match self.response_type {
            ResponseType::Info => response.has_info(),
            ResponseType::SetOption => response.has_set_option(),
            ResponseType::Query => response.has_query(),
            ResponseType::CheckTx => response.has_check_tx(),
            ResponseType::InitChain => response.has_init_chain(),
            ResponseType::BeginBlock => response.has_begin_block(),
            ResponseType::DeliverTx => response.has_deliver_tx(),
            ResponseType::EndBlock => response.has_end_block(),
            ResponseType::Commit => response.has_commit(),
            ResponseType::Flush => response.has_flush(),
            ResponseType::Echo => response.has_echo(),
            ResponseType::Exception => response.has_exception(),
        };
        if !has_expected {
            return Err(());
        }
        Ok(())
    }

    /// Respond with a response, consuming itself.
    /// Sends the response back to the server asynchronously, does not block.
    /// Checks that the response matches the request for which the responder was created.
    pub fn respond(self, response: Response) -> Result<(), ()> {
        if self.check_response(&response).is_err() {
            return Err(());
        }
        if self
            .response_sender
            .send((response, self.network_sender))
            .is_ok()
        {
            return Ok(());
        }
        Err(())
    }
}

/// Exit signal internally used both to stop the protocol and server.
#[derive(Eq, PartialEq)]
pub struct ExitSignal;

/// A signaling mechanism passed to the application on start-up,
/// to be used to signal the ABCI layer to exit.
#[derive(Clone)]
pub struct Exiter(Sender<ExitSignal>);

impl Exiter {
    /// Can be used by the application to signal to the ABCI layer to exit.
    pub fn exit(&self) -> Result<(), ()> {
        if self.0.send(ExitSignal).is_err() {
            return Err(());
        }
        Ok(())
    }
}

/// A layer between the application and the networking.
/// Could be used to keep track of order of requests/responses,
/// and provide other guarantees of sanity to the application.
struct Protocol {
    network_receiver: Receiver<(Request, oneshot::Sender<Response>)>,
    response_sender: Sender<(Response, oneshot::Sender<Response>)>,
    response_receiver: Receiver<(Response, oneshot::Sender<Response>)>,
    exit_receiver: Receiver<ExitSignal>,
    exit_sender: Sender<ExitSignal>,
    application: Box<dyn Application>,
    network_exit_sender: oneshot::Sender<ExitSignal>,
}

impl Protocol {
    fn new<A>(
        app: A,
        network_receiver: Receiver<(Request, oneshot::Sender<Response>)>,
        network_exit_sender: oneshot::Sender<ExitSignal>,
    ) -> Self
    where
        A: Application,
    {
        let (response_sender, response_receiver) = unbounded();
        let (exit_sender, exit_receiver) = unbounded();
        Protocol {
            application: Box::new(app),
            network_receiver,
            response_sender,
            response_receiver,
            exit_sender,
            exit_receiver,
            network_exit_sender,
        }
    }

    /// Handles messages from the network and application until exit.
    fn run(&mut self) -> bool {
        enum Incoming {
            Network((Request, oneshot::Sender<Response>)),
            Application((Response, oneshot::Sender<Response>)),
        }
        let incoming = select! {
            recv(self.exit_receiver) -> _ => return false,
            recv(self.response_receiver) -> msg => {
                msg.map(Incoming::Application).expect("Error in handling message from application")
            },
            recv(self.network_receiver) -> msg => {
                if msg.is_err() {
                    // Server disconnected, exit.
                    return false;
                }
                msg.map(Incoming::Network).expect("Error in handling message from network")
            },
        };
        match incoming {
            Incoming::Application((response, sender)) => {
                self.handle_application_response(response, sender)
            }
            Incoming::Network((request, sender)) => self.handle_network_request(request, sender),
        }
        true
    }

    /// Called once upon start-up.
    fn start(&mut self) {
        let exiter = Exiter(self.exit_sender.clone());
        self.application.start(exiter);
    }

    /// Called once upon exit.
    fn exit(mut self) {
        let _ = self.network_exit_sender.send(ExitSignal);
        self.application.exit();
    }

    /// Handle a message containing a response from the application.
    /// Could be used to perform additional sanity checks.
    /// Forwards the response to the network.
    fn handle_application_response(
        &mut self,
        response: Response,
        sender: oneshot::Sender<Response>,
    ) {
        // TODO: sanity checks on response.
        let _ = sender.send(response);
    }

    /// Handle a message containing a request from the network.
    /// Calls into the application directly as well, with the application either
    /// handling these calls in the current thread, or handling them in asynchronous fashion.
    fn handle_network_request(&mut self, request: Request, sender: oneshot::Sender<Response>) {
        // TODO: sanity checks on request.
        match request.value {
            // Info
            Some(Request_oneof_value::info(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::Info);
                self.application.info(r, responder);
            }
            // Init chain
            Some(Request_oneof_value::init_chain(r)) => {
                let responder = Responder::new(
                    self.response_sender.clone(),
                    sender,
                    ResponseType::InitChain,
                );
                self.application.init_chain(r, responder);
            }
            // Set option
            Some(Request_oneof_value::set_option(r)) => {
                let responder = Responder::new(
                    self.response_sender.clone(),
                    sender,
                    ResponseType::SetOption,
                );
                self.application.set_option(r, responder);
            }
            // Query
            Some(Request_oneof_value::query(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::Query);
                self.application.query(r, responder);
            }
            // Check tx
            Some(Request_oneof_value::check_tx(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::CheckTx);
                self.application.check_tx(r, responder);
            }
            // Begin block
            Some(Request_oneof_value::begin_block(r)) => {
                let responder = Responder::new(
                    self.response_sender.clone(),
                    sender,
                    ResponseType::BeginBlock,
                );
                self.application.begin_block(r, responder);
            }
            // Deliver Tx
            Some(Request_oneof_value::deliver_tx(r)) => {
                let responder = Responder::new(
                    self.response_sender.clone(),
                    sender,
                    ResponseType::DeliverTx,
                );
                self.application.deliver_tx(r, responder);
            }
            // End block
            Some(Request_oneof_value::end_block(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::EndBlock);
                self.application.end_block(r, responder);
            }
            // Commit
            Some(Request_oneof_value::commit(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::Commit);
                self.application.commit(r, responder);
            }
            // Flush
            Some(Request_oneof_value::flush(_)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::Flush);
                let mut response = Response::new();
                response.set_flush(ResponseFlush::new());
                let _ = responder.respond(response);
            }
            // Echo
            Some(Request_oneof_value::echo(r)) => {
                let responder =
                    Responder::new(self.response_sender.clone(), sender, ResponseType::Echo);
                let mut response = Response::new();
                let echo_msg = r.get_message().to_string();
                let mut echo = ResponseEcho::new();
                echo.set_message(echo_msg);
                response.set_echo(echo);
                let _ = responder.respond(response);
            }
            _ => {
                let responder = Responder::new(
                    self.response_sender.clone(),
                    sender,
                    ResponseType::Exception,
                );
                let mut response = Response::new();
                let mut re = ResponseException::new();
                re.set_error(String::from("Unrecognized request"));
                response.set_exception(re);
                let _ = responder.respond(response);
            }
        }
    }
}

/// Start the protocol, and the application.
/// Returns a sender of networking requests.
pub fn run_protocol<A>(
    app: A,
    network_exit_sender: oneshot::Sender<ExitSignal>,
) -> Sender<(Request, oneshot::Sender<Response>)>
where
    A: Application,
{
    let (network_sender, network_receiver) = unbounded();
    let mut protocol = Protocol::new(app, network_receiver, network_exit_sender);
    thread::spawn(move || {
        protocol.start();
        while protocol.run() {
            // running.
        }
        protocol.exit();
    });
    network_sender
}

/// Setup the app and start the server using localhost and default tendermint port 26658
/// Blocks the current thread.
pub fn run_local<A>(app: A)
where
    A: Application,
{
    let addr = "127.0.0.1:26658".parse().unwrap();
    run(addr, app);
}

/// Setup the application and start the server. Use this fn when setting different ip:port.
/// Blocks the current thread.
pub fn run<A>(listen_addr: SocketAddr, app: A)
where
    A: Application,
{
    serve(app, listen_addr).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_calls_app() {
        #[derive(Eq, PartialEq)]
        enum Checks {
            CalledStart,
            SentCheckTx,
            SentDeliverTx,
            CalledExit,
        }
        struct ChecksApp {
            checks_sender: Sender<Checks>,
            exiter: Option<Exiter>,
        };
        impl Application for ChecksApp {
            fn check_tx(&mut self, _req: RequestCheckTx, responder: Responder) {
                let mut response = Response::new();
                let res = ResponseCheckTx::new();
                response.set_check_tx(res);
                let _ = responder.respond(response);
                let _ = self.checks_sender.send(Checks::SentCheckTx);
            }
            fn deliver_tx(&mut self, _req: RequestDeliverTx, responder: Responder) {
                let mut response = Response::new();
                let res = ResponseDeliverTx::new();
                response.set_deliver_tx(res);
                let _ = responder.respond(response);
                let _ = self.checks_sender.send(Checks::SentDeliverTx);
            }
            fn start(&mut self, exiter: Exiter) {
                self.exiter = Some(exiter);
                let _ = self.checks_sender.send(Checks::CalledStart);
            }
            fn exit(&mut self) {
                let _ = self.checks_sender.send(Checks::CalledExit);
            }
        }

        // Start the protocol and app.
        let (checks_sender, checks_receiver) = unbounded();
        let app = ChecksApp {
            checks_sender,
            exiter: None,
        };
        let (exit_sender, mut exit_receiver) = oneshot::channel();
        let protocol_sender = run_protocol(app, exit_sender);

        // Send a check tx req.
        let mut request = Request::new();
        request.set_check_tx(RequestCheckTx::new());
        let (response_sender, _) = oneshot::channel();
        let _ = protocol_sender.send((request, response_sender));

        // Send a deliver tx req.
        let mut request = Request::new();
        request.set_deliver_tx(RequestDeliverTx::new());
        let (response_sender, _) = oneshot::channel();
        let _ = protocol_sender.send((request, response_sender));

        // Start receiving the first three checks.
        let mut counter = 0;
        while let Ok(msg) = checks_receiver.recv() {
            match counter {
                0 => assert!(msg == Checks::CalledStart),
                1 => assert!(msg == Checks::SentCheckTx),
                2 => {
                    assert!(msg == Checks::SentDeliverTx);
                    break;
                }
                _ => {}
            }
            counter += 1;
        }

        // Simulate an exiting server.
        drop(protocol_sender);

        // Check for a last exit message.
        let exit = checks_receiver.recv().expect("A last exit message");
        assert!(exit == Checks::CalledExit);

        // Check that the server received the exit message.
        loop {
            if let Ok(msg) = exit_receiver.try_recv() {
                assert!(msg == ExitSignal);
                break;
            }
        }
    }
}
