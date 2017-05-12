extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod types_grpc;
pub mod types;

use grpc::*;

use types_grpc::ABCIApplication;

pub struct DefaultABCIImplementation;

impl ABCIApplication for DefaultABCIImplementation {
    fn Echo(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestEcho) -> ::grpc::GrpcSingleResponse<types::ResponseEcho> {
        println!("Echo");
        let response = types::ResponseEcho::new();
        GrpcSingleResponse::completed(response)
    }

    fn Flush(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestFlush) -> ::grpc::GrpcSingleResponse<types::ResponseFlush> {
        println!("Flush");
        let response = types::ResponseFlush::new();
        GrpcSingleResponse::completed(response)
    }

    fn Info(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestInfo) -> ::grpc::GrpcSingleResponse<types::ResponseInfo> {
        println!("Info");
        let response = types::ResponseInfo::new();
        GrpcSingleResponse::completed(response)
    }

    fn SetOption(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestSetOption) -> ::grpc::GrpcSingleResponse<types::ResponseSetOption> {
        println!("SetOption");
        let response = types::ResponseSetOption::new();
        GrpcSingleResponse::completed(response)
    }

    fn DeliverTx(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestDeliverTx) -> ::grpc::GrpcSingleResponse<types::ResponseDeliverTx> {
        println!("DeliverTx");
        let response = types::ResponseDeliverTx::new();
        GrpcSingleResponse::completed(response)
    }

    fn CheckTx(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestCheckTx) -> ::grpc::GrpcSingleResponse<types::ResponseCheckTx> {
        println!("CheckTx");
        let response = types::ResponseCheckTx::new();
        GrpcSingleResponse::completed(response)
    }

    fn Query(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestQuery) -> ::grpc::GrpcSingleResponse<types::ResponseQuery> {
        println!("Query");
        let response = types::ResponseQuery::new();
        GrpcSingleResponse::completed(response)
    }

    fn Commit(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestCommit) -> ::grpc::GrpcSingleResponse<types::ResponseCommit> {
        println!("Commit");
        let response = types::ResponseCommit::new();
        GrpcSingleResponse::completed(response)
    }

    fn InitChain(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestInitChain) -> ::grpc::GrpcSingleResponse<types::ResponseInitChain> {
        println!("InitChain");
        let response = types::ResponseInitChain::new();
        GrpcSingleResponse::completed(response)
    }

    fn BeginBlock(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestBeginBlock) -> ::grpc::GrpcSingleResponse<types::ResponseBeginBlock> {
        println!("BeginBlock");
        let response = types::ResponseBeginBlock::new();
        GrpcSingleResponse::completed(response)
    }

    fn EndBlock(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestEndBlock) -> ::grpc::GrpcSingleResponse<types::ResponseEndBlock> {
        println!("EndBlock");
        let response = types::ResponseEndBlock::new();
        GrpcSingleResponse::completed(response)
    }
}


pub struct Server {}

impl Server {
    pub fn new(lAddr: &str, connectionType: &str) -> Option<Box<ABCIApplication>> {
        match connectionType {
            "grpc" => {
                println!("GRPC");
                Some(Box::new(DefaultABCIImplementation{}))
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
