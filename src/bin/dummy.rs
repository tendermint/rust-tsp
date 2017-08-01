#![allow(unused_variables)]
#![allow(unused_must_use)]
extern crate grpc;
extern crate rust_abci;
extern crate tokio_proto;

use rust_abci::types::*;
use rust_abci::types_grpc::ABCIApplication;
use rust_abci::socket_server::Application;


#[derive(Copy, Clone)]
struct DummyApp;

unsafe impl Sync for DummyApp {}

unsafe impl Send for DummyApp {}

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

// GRPC Implementation
impl ABCIApplication for DummyApp {
    fn echo(&self, o: ::grpc::RequestOptions, p: RequestEcho) -> ::grpc::SingleResponse<ResponseEcho> {
        println!("Echo");
        let response = ResponseEcho::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn flush(&self, o: ::grpc::RequestOptions, p: RequestFlush) -> ::grpc::SingleResponse<ResponseFlush> {
        println!("Flush");
        let response = ResponseFlush::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn info(&self, o: ::grpc::RequestOptions, p: RequestInfo) -> ::grpc::SingleResponse<ResponseInfo> {
        println!("Info");
        let response = ResponseInfo::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn set_option(&self, o: ::grpc::RequestOptions, p: RequestSetOption) -> ::grpc::SingleResponse<ResponseSetOption> {
        println!("SetOption");
        let response = ResponseSetOption::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn deliver_tx(&self, o: ::grpc::RequestOptions, p: RequestDeliverTx) -> ::grpc::SingleResponse<ResponseDeliverTx> {
        println!("DeliverTx");
        let response = ResponseDeliverTx::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn check_tx(&self, o: ::grpc::RequestOptions, p: RequestCheckTx) -> ::grpc::SingleResponse<ResponseCheckTx> {
        println!("CheckTx");
        let response = ResponseCheckTx::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn query(&self, o: ::grpc::RequestOptions, p: RequestQuery) -> ::grpc::SingleResponse<ResponseQuery> {
        println!("Query");
        let response = ResponseQuery::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn commit(&self, o: ::grpc::RequestOptions, p: RequestCommit) -> ::grpc::SingleResponse<ResponseCommit> {
        println!("Commit");
        let response = ResponseCommit::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn init_chain(&self, o: ::grpc::RequestOptions, p: RequestInitChain) -> ::grpc::SingleResponse<ResponseInitChain> {
        println!("InitChain");
        let response = ResponseInitChain::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn begin_block(&self, o: ::grpc::RequestOptions, p: RequestBeginBlock) -> ::grpc::SingleResponse<ResponseBeginBlock> {
        println!("BeginBlock");
        let response = ResponseBeginBlock::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn end_block(&self, o: ::grpc::RequestOptions, p: RequestEndBlock) -> ::grpc::SingleResponse<ResponseEndBlock> {
        println!("EndBlock");
        let response = ResponseEndBlock::new();
        ::grpc::SingleResponse::completed(response)
    }
}


fn main() {
    use std::env;
    use std::thread;

    let args: Vec<String> = env::args().collect();
    let connection_type: &str = &args[1];
    let listen_addr: &str = &args[2];


    match connection_type {
        "grpc" => rust_abci::grpc_server::new_server(listen_addr, DummyApp),
        "socket" => rust_abci::socket_server::new_server(listen_addr, DummyApp),
        _ => unimplemented!(),
    }

    loop {
        thread::park();
    }
}
