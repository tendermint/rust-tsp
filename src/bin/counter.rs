extern crate rust_abci;
extern crate grpc;
extern crate byteorder;

use std::thread;
use std::sync::Mutex;
use grpc::*;
use rust_abci::*;
use rust_abci::types_grpc::*;
use rust_abci::types;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

struct CounterApp {
    serial: Mutex<bool>,
    txCount: Mutex<u64>,
    hashCount: Mutex<u64>,
}

impl CounterApp {
    fn new(serial: bool) -> CounterApp {
        CounterApp {
            serial: Mutex::new(serial),
            txCount: Mutex::new(0),
            hashCount: Mutex::new(0),
        }
    }
}

impl ABCIApplication for CounterApp {
    fn Echo(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestEcho) -> ::grpc::GrpcSingleResponse<types::ResponseEcho> {
        let echo = p.get_message();
        let mut response = types::ResponseEcho::new();
        response.set_message(echo.to_owned());
        GrpcSingleResponse::completed(response)
    }

    fn Flush(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestFlush) -> ::grpc::GrpcSingleResponse<types::ResponseFlush> {
        unimplemented!();
    }

    fn Info(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestInfo) -> ::grpc::GrpcSingleResponse<types::ResponseInfo> {
        let mut response = types::ResponseInfo::new();
        response.set_data("CounterApp".to_owned());
        response.set_version("0.1.0".to_owned());
        GrpcSingleResponse::completed(response)
    }

    fn SetOption(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestSetOption) -> ::grpc::GrpcSingleResponse<types::ResponseSetOption> {
        let mut response = types::ResponseSetOption::new();
        if p.get_key() == "serial" && p.get_value() == "on" {
            let mut serial = self.serial.lock().unwrap();
            *serial = true;
            response.set_log("Serial set to ON".to_owned());
        }
        GrpcSingleResponse::completed(response)
    }

    fn DeliverTx(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestDeliverTx) -> ::grpc::GrpcSingleResponse<types::ResponseDeliverTx> {
        let mut response = types::ResponseDeliverTx::new();
        if *self.serial.lock().unwrap() {
            if p.get_tx().len() > 8 {
                response.set_code(types::CodeType::EncodingError);
                response.set_log("Max tx size is 8 bytes".to_owned());
                return GrpcSingleResponse::completed(response);
            }
        }
        let nonce = p.get_tx().read_uint::<BigEndian>(p.get_tx().len()).unwrap();
        if *self.txCount.lock().unwrap() != nonce {
            response.set_code(types::CodeType::BadNonce);
            response.set_log("Invalid nonce.".to_owned());
            return GrpcSingleResponse::completed(response);
        }
        let mut txCount = self.txCount.lock().unwrap();
        *txCount += 1;
        response.set_code(types::CodeType::OK);
        GrpcSingleResponse::completed(response)
    }

    fn CheckTx(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestCheckTx) -> ::grpc::GrpcSingleResponse<types::ResponseCheckTx> {
        let mut response = types::ResponseCheckTx::new();
        if *self.serial.lock().unwrap() {
            if p.get_tx().len() > 8 {
                response.set_code(types::CodeType::EncodingError);
                response.set_log("Max tx size is 8 bytes".to_owned());
                return GrpcSingleResponse::completed(response);
            }
        }
        let nonce = p.get_tx().read_uint::<BigEndian>(p.get_tx().len()).unwrap();
        if *self.txCount.lock().unwrap() != nonce {
            response.set_code(types::CodeType::BadNonce);
            response.set_log("Invalid nonce.".to_owned());
            return GrpcSingleResponse::completed(response);
        }
        response.set_code(types::CodeType::OK);
        GrpcSingleResponse::completed(response)
    }

    fn Query(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestQuery) -> ::grpc::GrpcSingleResponse<types::ResponseQuery> {
        let mut response = types::ResponseQuery::new();
        match p.get_path() {
            "hash" => {
                let mut data = vec![];
                data.write_uint::<BigEndian>(*self.hashCount.lock().unwrap(), 8);
                response.set_value(data);
                return GrpcSingleResponse::completed(response);
            },
            "tx" => {
                let mut data = vec![];
                data.write_uint::<BigEndian>(*self.txCount.lock().unwrap(), 8);
                response.set_value(data);
                return GrpcSingleResponse::completed(response);
            },
            _ => {
                response.set_log("Invalid query path. Expected hash or tx.".to_owned());
                return GrpcSingleResponse::completed(response);
            },
        }
    }

    fn Commit(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestCommit) -> ::grpc::GrpcSingleResponse<types::ResponseCommit> {
        let mut response = types::ResponseCommit::new();

        let mut hashCount = self.hashCount.lock().unwrap();
        *hashCount += 1;

        if *self.txCount.lock().unwrap() == 0 {
            response.set_code(types::CodeType::OK);
            return GrpcSingleResponse::completed(response);
        }

        let mut data = vec![];
        data.write_uint::<BigEndian>(*self.txCount.lock().unwrap(), 8);
        response.set_data(data);
        GrpcSingleResponse::completed(response)
    }

    fn InitChain(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestInitChain) -> ::grpc::GrpcSingleResponse<types::ResponseInitChain> {
        let response = types::ResponseInitChain::new();
        GrpcSingleResponse::completed(response)
    }

    fn BeginBlock(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestBeginBlock) -> ::grpc::GrpcSingleResponse<types::ResponseBeginBlock> {
        let response = types::ResponseBeginBlock::new();
        GrpcSingleResponse::completed(response)
    }

    fn EndBlock(&self, o: ::grpc::GrpcRequestOptions, p: types::RequestEndBlock) -> ::grpc::GrpcSingleResponse<types::ResponseEndBlock> {
        let response = types::ResponseEndBlock::new();
        GrpcSingleResponse::completed(response)
    }
}

fn main() {
    let lAddr = "0.0.0.0:46658";
    let connectionType = "grpc";

    let app = CounterApp::new(true);

    let _server = NewServer(lAddr, connectionType, app);

    loop {
        thread::park();
    }
}