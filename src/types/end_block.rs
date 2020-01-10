use crate::proto::abci::{
    Event as ProtoEvent, RequestEndBlock, ResponseEndBlock, ValidatorUpdate as ProtoValidatorUpdate,
};
use crate::types::{ConsensusParams, Event, ValidatorUpdate};

#[derive(Debug, Default)]
pub struct EndBlockRequest {
    /// Height of the block just executed
    pub height: i64,
}

impl From<RequestEndBlock> for EndBlockRequest {
    fn from(request_end_block: RequestEndBlock) -> EndBlockRequest {
        EndBlockRequest {
            height: request_end_block.height,
        }
    }
}

#[derive(Debug, Default)]
pub struct EndBlockResponse {
    /// Changes to validator set (set voting power to 0 to remove)
    pub validator_updates: Vec<ValidatorUpdate>,
    /// Changes to consensus-critical time, size, and other parameters
    pub consensus_param_updates: Option<ConsensusParams>,
    /// Events for filtering and indexing
    pub events: Vec<Event>,
}

impl From<EndBlockResponse> for ResponseEndBlock {
    fn from(end_block_response: EndBlockResponse) -> ResponseEndBlock {
        let mut response_end_block = ResponseEndBlock::new();
        response_end_block.validator_updates = end_block_response
            .validator_updates
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoValidatorUpdate>>()
            .into();
        response_end_block.consensus_param_updates = end_block_response
            .consensus_param_updates
            .map(Into::into)
            .into();
        response_end_block.events = end_block_response
            .events
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoEvent>>()
            .into();
        response_end_block
    }
}
