use crate::proto::abci::{Event as ProtoEvent, RequestBeginBlock, ResponseBeginBlock};
use crate::types::{Event, Evidence, Header, LastCommitInfo};

#[derive(Debug, Default)]
pub struct BeginBlockRequest {
    /// Block's hash. This can be derived from the block header
    pub hash: Vec<u8>,
    /// Block header
    pub header: Option<Header>,
    /// Info about the last commit, including the round, and the list of validators and which ones signed the last block
    pub last_commit_info: Option<LastCommitInfo>,
    /// List of evidence of validators that acted maliciously
    pub byzantine_validators: Vec<Evidence>,
}

impl From<RequestBeginBlock> for BeginBlockRequest {
    fn from(request_begin_block: RequestBeginBlock) -> BeginBlockRequest {
        BeginBlockRequest {
            hash: request_begin_block.hash,
            header: request_begin_block.header.into_option().map(Into::into),
            last_commit_info: request_begin_block
                .last_commit_info
                .into_option()
                .map(Into::into),
            byzantine_validators: request_begin_block
                .byzantine_validators
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[derive(Debug, Default)]
pub struct BeginBlockResponse {
    /// Events for filtering and indexing
    pub events: Vec<Event>,
}

impl From<BeginBlockResponse> for ResponseBeginBlock {
    fn from(begin_block_response: BeginBlockResponse) -> ResponseBeginBlock {
        let mut response_begin_block = ResponseBeginBlock::new();
        response_begin_block.events = begin_block_response
            .events
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoEvent>>()
            .into();
        response_begin_block
    }
}
