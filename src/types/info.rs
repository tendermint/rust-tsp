use crate::proto::abci::{RequestInfo, ResponseInfo};

#[derive(Debug, Default)]
pub struct InfoRequest {
    /// Tendermint software semantic version
    pub version: String,
    /// Tendermint block protocol version
    pub block_version: u64,
    /// Tendermint P2P protocol version
    pub p2p_version: u64,
}

impl From<RequestInfo> for InfoRequest {
    fn from(request_info: RequestInfo) -> InfoRequest {
        InfoRequest {
            version: request_info.version,
            block_version: request_info.block_version,
            p2p_version: request_info.p2p_version,
        }
    }
}

#[derive(Debug, Default)]
pub struct InfoResponse {
    /// Some arbitrary information
    pub data: String,
    /// Application software semantic version
    pub version: String,
    /// Application protocol version
    pub app_version: u64,
    /// Latest block for which the app has called Commit
    pub last_block_height: i64,
    /// Latest result of Commit
    pub last_block_app_hash: Vec<u8>,
}

impl From<InfoResponse> for ResponseInfo {
    fn from(info_response: InfoResponse) -> ResponseInfo {
        let mut response_info = ResponseInfo::new();
        response_info.data = info_response.data;
        response_info.version = info_response.version;
        response_info.app_version = info_response.app_version;
        response_info.last_block_height = info_response.last_block_height;
        response_info.last_block_app_hash = info_response.last_block_app_hash;
        response_info
    }
}
