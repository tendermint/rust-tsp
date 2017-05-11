extern crate rust_abci;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

use std::thread;

use grpc::*;

use futures_cpupool::CpuPool;

use rust_abci::*;
use rust_abci::types_grpc::*;
use rust_abci::types;

struct DummyImpl;

impl ABCIApplication for DummyImpl {
    fn Echo(&self, o: GrpcRequestOptions, p: types::RequestEcho) -> GrpcSingleResponse<types::ResponseEcho> {
        println!("Test");
        unimplemented!();
    }

    fn Flush(&self, o: GrpcRequestOptions, p: types::RequestFlush) -> GrpcSingleResponse<types::ResponseFlush> {
        unimplemented!();
    }

    fn Info(&self, o: GrpcRequestOptions, p: types::RequestInfo) -> GrpcSingleResponse<types::ResponseInfo> {
        unimplemented!();
    }

    fn SetOption(&self, o: GrpcRequestOptions, p: types::RequestSetOption) -> GrpcSingleResponse<types::ResponseSetOption> {
        unimplemented!();
    }

    fn DeliverTx(&self, o: GrpcRequestOptions, p: types::RequestDeliverTx) -> GrpcSingleResponse<types::ResponseDeliverTx> {
        unimplemented!();
    }

    fn CheckTx(&self, o: GrpcRequestOptions, p: types::RequestCheckTx) -> GrpcSingleResponse<types::ResponseCheckTx> {
        unimplemented!();
    }

    fn Query(&self, o: GrpcRequestOptions, p: types::RequestQuery) -> GrpcSingleResponse<types::ResponseQuery> {
        unimplemented!();
    }

    fn Commit(&self, o: GrpcRequestOptions, p: types::RequestCommit) -> GrpcSingleResponse<types::ResponseCommit> {
        unimplemented!();
    }

    fn InitChain(&self, o: GrpcRequestOptions, p: types::RequestInitChain) -> GrpcSingleResponse<types::ResponseInitChain> {
        unimplemented!();
    }

    fn BeginBlock(&self, o: GrpcRequestOptions, p: types::RequestBeginBlock) -> GrpcSingleResponse<types::ResponseBeginBlock> {
        unimplemented!();
    }

    fn EndBlock(&self, o: GrpcRequestOptions, p: types::RequestEndBlock) -> GrpcSingleResponse<types::ResponseEndBlock> {
        unimplemented!();
    }
}


fn main() {
    println!("{:?}", rust_abci::hello());

    let lAddr = "0.0.0.0:46658";
    let connectionType = "grpc";

    let _server = ABCIApplicationServer::new_pool("[::]:46658", Default::default(), DummyImpl, CpuPool::new(4));

    loop {
        thread::park();
    }
}