extern crate tsp;
extern crate tokio_proto;

use std::env;
use std::thread;

use tsp::types::*;
use tsp::Application;


#[derive(Copy, Clone)]
struct DummyApp;

// Socket implementation
impl Application for DummyApp {
    fn begin_block(&self, p: &RequestBeginBlock) -> ResponseBeginBlock {
        println!("begin_block");
        ResponseBeginBlock::new()
    }

    fn check_tx(&self, p: &RequestCheckTx) -> ResponseCheckTx {
        println!("check_tx");
        ResponseCheckTx::new()
    }

    fn commit(&self, p: &RequestCommit) -> ResponseCommit {
        println!("commit");
        ResponseCommit::new()
    }

    fn deliver_tx(&self, p: &RequestDeliverTx) -> ResponseDeliverTx {
        println!("deliver_tx");
        ResponseDeliverTx::new()
    }

    fn echo(&self, p: &RequestEcho) -> ResponseEcho {
        println!("echo");
        let mut response = ResponseEcho::new();
        response.set_message(p.get_message().to_owned());
        return response;
    }

    fn end_block(&self, p: &RequestEndBlock) -> ResponseEndBlock {
        println!("end_block");
        ResponseEndBlock::new()
    }

    fn flush(&self, p: &RequestFlush) -> ResponseFlush {
        println!("flush");
        ResponseFlush::new()
    }

    fn init_chain(&self, p: &RequestInitChain) -> ResponseInitChain {
        println!("init_chain");
        ResponseInitChain::new()
    }

    fn info(&self, p: &RequestInfo) -> ResponseInfo {
        println!("info");
        ResponseInfo::new()
    }

    fn query(&self, p: &RequestQuery) -> ResponseQuery {
        println!("query");
        ResponseQuery::new()
    }

    fn set_option(&self, p: &RequestSetOption) -> ResponseSetOption {
        println!("set_option");
        ResponseSetOption::new()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let connection_type: &str = &args[1];
    let listen_addr: &str = &args[2];
    static APP: DummyApp = DummyApp;

    match connection_type {
        "socket" => tsp::server::new(listen_addr.parse().unwrap(), &APP),
        _ => unimplemented!(),
    }

    loop {
        thread::park();
    }
}
