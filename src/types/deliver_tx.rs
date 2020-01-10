use crate::proto::abci::{Event as ProtoEvent, RequestDeliverTx, ResponseDeliverTx};
use crate::types::{Event, Result};

#[derive(Debug, Default)]
pub struct DeliverTxRequest {
    /// The request transaction bytes
    pub tx: Vec<u8>,
}

impl From<RequestDeliverTx> for DeliverTxRequest {
    fn from(request_deliver_tx: RequestDeliverTx) -> DeliverTxRequest {
        DeliverTxRequest {
            tx: request_deliver_tx.tx,
        }
    }
}

#[derive(Debug, Default)]
pub struct DeliverTxResponse {
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

impl From<Result<DeliverTxResponse>> for ResponseDeliverTx {
    fn from(deliver_tx_response: Result<DeliverTxResponse>) -> ResponseDeliverTx {
        let mut response_deliver_tx = ResponseDeliverTx::new();

        match deliver_tx_response {
            Ok(deliver_tx_response) => {
                response_deliver_tx.data = deliver_tx_response.data;
                response_deliver_tx.log = deliver_tx_response.log;
                response_deliver_tx.info = deliver_tx_response.info;
                response_deliver_tx.gas_wanted = deliver_tx_response.gas_wanted;
                response_deliver_tx.gas_used = deliver_tx_response.gas_used;
                response_deliver_tx.events = deliver_tx_response
                    .events
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<ProtoEvent>>()
                    .into();
            }
            Err(error) => {
                response_deliver_tx.code = error.code;
                response_deliver_tx.codespace = error.codespace;
                response_deliver_tx.log = error.log;
                response_deliver_tx.info = error.info;
            }
        }

        response_deliver_tx
    }
}
