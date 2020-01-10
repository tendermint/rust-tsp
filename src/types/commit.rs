use crate::proto::abci::ResponseCommit;

#[derive(Debug, Default)]
pub struct CommitResponse {
    /// The Merkle root hash of the application state
    pub data: Vec<u8>,
}

impl From<CommitResponse> for ResponseCommit {
    fn from(commit_response: CommitResponse) -> ResponseCommit {
        let mut response_commit = ResponseCommit::new();
        response_commit.data = commit_response.data;
        response_commit
    }
}
