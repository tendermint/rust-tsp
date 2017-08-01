use futures_cpupool::CpuPool;

use types_grpc::{ABCIApplication, ABCIApplicationServer};


pub fn new_server<H: ABCIApplication + 'static + Sync + Send>(listen_addr: String, app: H) {
    ABCIApplicationServer::new_pool(listen_addr, Default::default(), app, CpuPool::new(4)).unwrap();
}
