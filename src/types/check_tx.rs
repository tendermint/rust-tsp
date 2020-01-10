use crate::proto::abci::{
    CheckTxType as ProtoCheckTxType, Event as ProtoEvent, RequestCheckTx, ResponseCheckTx,
};
use crate::types::{Event, Result};

#[derive(Debug, Default)]
pub struct CheckTxRequest {
    /// The request transaction bytes
    pub tx: Vec<u8>,
    /// Denotes if this is a new request of a re-check request
    pub check_type: CheckTxType,
}

impl From<RequestCheckTx> for CheckTxRequest {
    fn from(request_check_tx: RequestCheckTx) -> CheckTxRequest {
        CheckTxRequest {
            tx: request_check_tx.tx,
            check_type: request_check_tx.field_type.into(),
        }
    }
}

#[derive(Debug)]
pub enum CheckTxType {
    /// Denotes that the transaction has never been checked
    New,
    /// Denotes that the transaction was already checked and certain expensive operation (like checking signatures) can
    /// be skipped
    Recheck,
}

impl Default for CheckTxType {
    #[inline]
    fn default() -> Self {
        Self::New
    }
}

impl From<ProtoCheckTxType> for CheckTxType {
    fn from(proto_check_tx_type: ProtoCheckTxType) -> Self {
        match proto_check_tx_type {
            ProtoCheckTxType::New => Self::New,
            ProtoCheckTxType::Recheck => Self::Recheck,
        }
    }
}

#[derive(Debug, Default)]
pub struct CheckTxResponse {
    /// Result bytes, if any.
    pub data: Vec<u8>,
    /// Output of application's logger (may be non-deterministic)
    pub log: String,
    /// Additional information (may be non-deterministic)
    pub info: String,
    /// Amount of gas requested for transaction
    pub gas_wanted: i64,
    /// Amount of gas consumed by transaction
    pub gas_used: i64,
    /// Events for filtering and indexing
    pub events: Vec<Event>,
}

impl From<Result<CheckTxResponse>> for ResponseCheckTx {
    fn from(check_tx_response: Result<CheckTxResponse>) -> ResponseCheckTx {
        let mut response_check_tx = ResponseCheckTx::new();

        match check_tx_response {
            Ok(check_tx_response) => {
                response_check_tx.data = check_tx_response.data;
                response_check_tx.log = check_tx_response.log;
                response_check_tx.info = check_tx_response.info;
                response_check_tx.gas_wanted = check_tx_response.gas_wanted;
                response_check_tx.gas_used = check_tx_response.gas_used;
                response_check_tx.events = check_tx_response
                    .events
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<ProtoEvent>>()
                    .into();
            }
            Err(error) => {
                response_check_tx.code = error.code;
                response_check_tx.codespace = error.codespace;
                response_check_tx.log = error.log;
                response_check_tx.info = error.info;
            }
        }

        response_check_tx
    }
}
