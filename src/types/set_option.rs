use crate::proto::abci::{RequestSetOption, ResponseSetOption};
use crate::types::Result;

#[derive(Debug, Default)]
pub struct SetOptionRequest {
    /// Key to set
    pub key: String,
    /// Value to set for key
    pub value: String,
}

impl From<RequestSetOption> for SetOptionRequest {
    fn from(request_set_option: RequestSetOption) -> SetOptionRequest {
        SetOptionRequest {
            key: request_set_option.key,
            value: request_set_option.value,
        }
    }
}

#[derive(Debug, Default)]
pub struct SetOptionResponse {
    /// Output of application's logger (may be non-deterministic)
    pub log: String,
    /// Additional information (may be non-deterministic)
    pub info: String,
}

impl From<Result<SetOptionResponse>> for ResponseSetOption {
    fn from(set_option_response: Result<SetOptionResponse>) -> ResponseSetOption {
        let mut response_set_option = ResponseSetOption::new();

        match set_option_response {
            Ok(set_option_response) => {
                response_set_option.log = set_option_response.log;
                response_set_option.info = set_option_response.info;
            }
            Err(error) => {
                response_set_option.code = error.code;
                response_set_option.log = error.log;
                response_set_option.info = error.info;
            }
        }

        response_set_option
    }
}
