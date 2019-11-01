use crossbeam_channel::Sender;
use env_logger::Env;
use std::net::SocketAddr;
use tokio;
use tokio::codec::Decoder;
use tokio::io;
use tokio::net::{tcp::Incoming, TcpListener};
use tokio::prelude::*;
use tokio::runtime::current_thread;
use tokio::sync::oneshot;

use crate::codec::ABCICodec;
use crate::messages::abci::*;
use crate::{run_protocol, Application, ExitSignal};

/// Creates the TCP server and listens for connections from Tendermint
pub fn serve<A>(app: A, addr: SocketAddr) -> io::Result<()>
where
    A: Application,
{
    env_logger::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .ok();
    let listener = TcpListener::bind(&addr).unwrap();
    let (exit_sender, exit_receiver) = oneshot::channel();
    let protocol_sender = run_protocol(app, exit_sender);
    let incoming = listener.incoming();
    let mut runtime = current_thread::Runtime::new().expect("To start a runtime");
    let server = Server {
        runtime_handle: runtime.handle(),
        incoming,
        protocol_sender,
        exit_receiver,
    };

    runtime
        .block_on(server)
        .expect("Runtime to block on server");
    Ok(())
}

struct Server {
    runtime_handle: current_thread::Handle,
    incoming: Incoming,
    protocol_sender: Sender<(Request, oneshot::Sender<Response>)>,
    exit_receiver: oneshot::Receiver<ExitSignal>,
}

struct Connection {
    runtime_handle: current_thread::Handle,
    protocol_sender: Sender<(Request, oneshot::Sender<Response>)>,
    response_receiver: oneshot::Receiver<Response>,
    response_sender: Option<oneshot::Sender<Response>>,
    writer: Option<stream::SplitSink<tokio::codec::Framed<tokio::net::TcpStream, ABCICodec>>>,
    reader: Option<stream::SplitStream<tokio::codec::Framed<tokio::net::TcpStream, ABCICodec>>>,
}

impl Future for Server {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.exit_receiver.poll() {
            Ok(Async::Ready(_)) => return Ok(Async::Ready(())),
            Ok(Async::NotReady) => {}
            Err(_) => return Err(()),
        }
        match self.incoming.poll() {
            Ok(Async::Ready(Some(socket))) => {
                let framed = ABCICodec::new().framed(socket);
                let (writer, reader) = framed.split();
                let (sender, response_receiver) = oneshot::channel();
                let connection = Connection {
                    runtime_handle: self.runtime_handle.clone(),
                    protocol_sender: self.protocol_sender.clone(),
                    response_receiver,
                    response_sender: Some(sender),
                    writer: Some(writer),
                    reader: Some(reader),
                };
                self.runtime_handle
                    .spawn(connection.then(|_| Ok(())))
                    .expect("To spawn a connection");
            }
            Ok(Async::NotReady) => {}
            Err(_) | Ok(Async::Ready(None)) => {
                // Connection closed, I assume this will drop the Server struct,
                // which will also shutdown the protocol thread,
                // when all protocol senders drop.
                return Ok(Async::Ready(()));
            }
        }
        Ok(Async::NotReady)
    }
}

impl Future for Connection {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        if let Some(reader) = &mut self.reader {
            match reader.poll() {
                Ok(Async::Ready(Some(request))) => {
                    let _ = self.protocol_sender.send((
                        request,
                        self.response_sender.take().expect("To have a sender"),
                    ));
                    self.reader = None;
                }
                Ok(Async::Ready(None)) => {
                    self.reader = None;
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(_) => return Err(()),
            }
        }
        if self.writer.is_some() {
            match self.response_receiver.poll() {
                Ok(Async::Ready(response)) => {
                    let writer = self.writer.take().expect("To have a writer");
                    let writes = writer.send(response);
                    self.runtime_handle
                        .spawn(writes.then(|_| Ok(())))
                        .expect("To spawn a writer");
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(_) => return Err(()),
            }
        }
        Ok(Async::Ready(()))
    }
}
