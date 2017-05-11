extern crate rust_abci;
extern crate grpc;

use std::thread;

use grpc::*;

use rust_abci::*;
use rust_abci::types_grpc::*;
use rust_abci::types::*;

struct DummyImpl {}

impl ABCIApplication for DummyImpl {
    fn Echo(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEcho) -> ::grpc::GrpcSingleResponse<super::types::ResponseEcho> {
        println!("Test");
        unimplemented!();
    }

    fn Flush(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestFlush) -> ::grpc::GrpcSingleResponse<super::types::ResponseFlush> {
        unimplemented!();
    }

    fn Info(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInfo) -> ::grpc::GrpcSingleResponse<super::types::ResponseInfo> {
        unimplemented!();
    }

    fn SetOption(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestSetOption) -> ::grpc::GrpcSingleResponse<super::types::ResponseSetOption> {
        unimplemented!();
    }

    fn DeliverTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestDeliverTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseDeliverTx> {
        unimplemented!();
    }

    fn CheckTx(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCheckTx) -> ::grpc::GrpcSingleResponse<super::types::ResponseCheckTx> {
        unimplemented!();
    }

    fn Query(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestQuery) -> ::grpc::GrpcSingleResponse<super::types::ResponseQuery> {
        unimplemented!();
    }

    fn Commit(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestCommit) -> ::grpc::GrpcSingleResponse<super::types::ResponseCommit> {
        unimplemented!();
    }

    fn InitChain(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestInitChain) -> ::grpc::GrpcSingleResponse<super::types::ResponseInitChain> {
        unimplemented!();
    }

    fn BeginBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestBeginBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseBeginBlock> {
        unimplemented!();
    }

    fn EndBlock(&self, o: ::grpc::GrpcRequestOptions, p: super::types::RequestEndBlock) -> ::grpc::GrpcSingleResponse<super::types::ResponseEndBlock> {
        unimplemented!();
    }
}


fn main() {
    println!("{:?}", rust_abci::hello());

    let lAddr = "0.0.0.0:46658";
    let connectionType = "grpc";

    let _server = Server::new(lAddr, connectionType);

    loop {
        thread::park();
    }
}