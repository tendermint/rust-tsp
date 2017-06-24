use futures_cpupool::CpuPool;
use types_grpc::{ABCIApplication, ABCIApplicationServer};
use super::Service;

impl Service for ABCIApplicationServer {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, app: H) -> Box<Service> {
    Box::new(ABCIApplicationServer::new_pool(listen_addr, Default::default(), app, CpuPool::new(4)).unwrap())
}
