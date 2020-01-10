use std::time::Duration;

use crate::proto::abci::{
    RequestInitChain, ResponseInitChain, ValidatorUpdate as ProtoValidatorUpdate,
};
use crate::types::{ConsensusParams, ValidatorUpdate};

#[derive(Debug, Default)]
pub struct InitChainRequest {
    /// Genesis time (duration since epoch)
    pub time: Option<Duration>,
    /// ID of blockchain
    pub chain_id: String,
    /// Initial consensus-critical parameters
    pub consensus_params: Option<ConsensusParams>,
    /// Initial genesis validators
    pub validators: Vec<ValidatorUpdate>,
    /// Serialized initial application state (amino-encoded JSON bytes)
    pub app_state_bytes: Vec<u8>,
}

impl From<RequestInitChain> for InitChainRequest {
    fn from(request_init_chain: RequestInitChain) -> InitChainRequest {
        InitChainRequest {
            time: request_init_chain
                .time
                .into_option()
                .map(|timestamp| Duration::new(timestamp.seconds as u64, timestamp.nanos as u32)),
            chain_id: request_init_chain.chain_id,
            consensus_params: request_init_chain
                .consensus_params
                .into_option()
                .map(Into::into),
            validators: request_init_chain
                .validators
                .into_iter()
                .map(Into::into)
                .collect(),
            app_state_bytes: request_init_chain.app_state_bytes,
        }
    }
}

#[derive(Debug, Default)]
pub struct InitChainResponse {
    /// Initial consensus-critical parameters
    pub consensus_params: Option<ConsensusParams>,
    /// Initial validator set (if non empty)
    pub validators: Vec<ValidatorUpdate>,
}

impl From<InitChainResponse> for ResponseInitChain {
    fn from(init_chain_response: InitChainResponse) -> ResponseInitChain {
        let mut response_init_chain = ResponseInitChain::new();
        response_init_chain.consensus_params =
            init_chain_response.consensus_params.map(Into::into).into();
        response_init_chain.validators = init_chain_response
            .validators
            .into_iter()
            .map(Into::into)
            .collect::<Vec<ProtoValidatorUpdate>>()
            .into();
        response_init_chain
    }
}
