extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod types_grpc;
pub mod types;

use grpc::*;
use types_grpc::{ABCIApplication, ABCIApplicationServer};
use futures_cpupool::CpuPool;

pub trait Service {}

impl Service for ABCIApplicationServer {}

pub fn NewServer<H: ABCIApplication + 'static + Sync + Send + 'static>(lAddr: &str, connectionType: &str, app: H) -> Option<Box<Service>> {
    match connectionType {
        "grpc" => {
            println!("GRPC");
            Some(Box::new(ABCIApplicationServer::new_pool(lAddr, Default::default(), app, CpuPool::new(4))))
        },
        "socket" => {
            println!("SOCKET");
            None
        },
        _ => {
            println!("UNKNOWN");
            None
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
