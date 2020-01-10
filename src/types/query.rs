use crate::proto::abci::{RequestQuery, ResponseQuery};
use crate::types::{Proof, Result};

#[derive(Debug, Default)]
pub struct QueryRequest {
    /// Raw query bytes (can be used with or in lieu of `path`)
    pub data: Vec<u8>,
    /// Path of request, like an HTTP GET path (can be used with or in lieu of `data`)
    ///
    /// # Note
    ///
    /// - Apps MUST interpret '/store' as a query by key on the underlying store. The key SHOULD be specified in the
    ///   `data` field
    /// - Apps SHOULD allow queries over specific types like '/accounts/...' or '/votes/...'
    pub path: String,
    /// Block height for which you want the query (default=0 returns data for the latest committed block)
    ///
    /// # Note
    ///
    /// This is the height of the block containing the application's Merkle root hash, which represents the state as it
    /// was after committing the block at `height-1`
    pub height: i64,
    /// Return Merkle proof with response if possible
    pub prove: bool,
}

impl From<RequestQuery> for QueryRequest {
    fn from(request_query: RequestQuery) -> QueryRequest {
        QueryRequest {
            data: request_query.data,
            path: request_query.path,
            height: request_query.height,
            prove: request_query.prove,
        }
    }
}

#[derive(Debug, Default)]
pub struct QueryResponse {
    /// Output of application's logger (may be non-deterministic)
    pub log: String,
    /// Additional information (may be non-deterministic)
    pub info: String,
    /// Index of the key in the tree
    pub index: i64,
    /// Key of the matching data
    pub key: Vec<u8>,
    /// Value of the matching data
    pub value: Vec<u8>,
    /// Serialized proof for the value data, if requested, to be verified against the app_hash for the given height
    pub proof: Option<Proof>,
    /// Block height from which data was derived
    ///
    /// # Note
    ///
    /// this is the height of the block containing the application's Merkle root hash, which represents the state as it
    /// was after committing the block at `height-1`
    pub height: i64,
}

impl From<Result<QueryResponse>> for ResponseQuery {
    fn from(query_response: Result<QueryResponse>) -> ResponseQuery {
        let mut response_query = ResponseQuery::new();

        match query_response {
            Ok(query_response) => {
                response_query.log = query_response.log;
                response_query.info = query_response.info;
                response_query.index = query_response.index;
                response_query.key = query_response.key;
                response_query.value = query_response.value;
                response_query.proof = query_response.proof.map(Into::into).into();
                response_query.height = query_response.height;
            }
            Err(error) => {
                response_query.code = error.code;
                response_query.codespace = error.codespace;
                response_query.log = error.log;
                response_query.info = error.info;
            }
        }

        response_query
    }
}
