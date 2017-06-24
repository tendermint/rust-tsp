//! This is the documentation for the rust-abci crate.

extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;
extern crate websocket;
extern crate tokio_core;
extern crate byteorder;


use types_grpc::ABCIApplication;


pub mod types;
pub mod types_grpc;
pub mod grpc_server;
pub mod socket_server;
pub mod wire;

pub trait Service {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, connection_type: &str, app: H) -> Option<Box<Service>> {
    match connection_type {
        "grpc" => Some(grpc_server::new_server(listen_addr, app)),
        "socket" => Some(socket_server::new_server(listen_addr, app)),
        _ => None,
    }
}
