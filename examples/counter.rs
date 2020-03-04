use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use abci::{async_trait, types::*, Consensus, Info, Mempool, Server};

/// Simple counter
#[derive(Debug, Default, Clone)]
pub struct CounterState {
    block_height: i64,
    app_hash: Vec<u8>,
    counter: u64,
}

#[derive(Debug)]
pub struct ConsensusConnection {
    committed_state: Arc<Mutex<CounterState>>,
    current_state: Arc<Mutex<Option<CounterState>>>,
}

impl ConsensusConnection {
    pub fn new(
        committed_state: Arc<Mutex<CounterState>>,
        current_state: Arc<Mutex<Option<CounterState>>>,
    ) -> Self {
        Self {
            committed_state,
            current_state,
        }
    }
}

#[async_trait]
impl Consensus for ConsensusConnection {
    async fn init_chain(&self, _init_chain_request: InitChainRequest) -> InitChainResponse {
        Default::default()
    }

    async fn begin_block(&self, _begin_block_request: BeginBlockRequest) -> BeginBlockResponse {
        let committed_state = self.committed_state.lock().unwrap().clone();

        let mut current_state = self.current_state.lock().unwrap();
        *current_state = Some(committed_state);

        Default::default()
    }

    async fn deliver_tx(&self, deliver_tx_request: DeliverTxRequest) -> Result<DeliverTxResponse> {
        let new_counter = parse_bytes_to_counter(&deliver_tx_request.tx)?;

        let mut current_state_lock = self.current_state.lock().unwrap();
        let mut current_state = current_state_lock.as_mut().unwrap();

        if current_state.counter + 1 != new_counter {
            return Err(Error {
                code: 2,
                codespace: "Validation error".to_owned(),
                log: "Only consecutive integers are allowed".to_owned(),
                info: "Numbers to counter app should be supplied in increasing order of consecutive integers staring from 1".to_owned(),
            });
        }

        current_state.counter = new_counter;

        Ok(Default::default())
    }

    async fn end_block(&self, end_block_request: EndBlockRequest) -> EndBlockResponse {
        let mut current_state_lock = self.current_state.lock().unwrap();
        let mut current_state = current_state_lock.as_mut().unwrap();

        current_state.block_height = end_block_request.height;
        current_state.app_hash = current_state.counter.to_be_bytes().to_vec();

        Default::default()
    }

    async fn commit(&self) -> CommitResponse {
        let current_state = self.current_state.lock().unwrap().as_ref().unwrap().clone();
        let mut committed_state = self.committed_state.lock().unwrap();
        *committed_state = current_state;

        CommitResponse {
            data: (*committed_state).app_hash.clone(),
        }
    }
}

#[derive(Debug)]
pub struct MempoolConnection {
    state: Arc<Mutex<Option<CounterState>>>,
}

impl MempoolConnection {
    pub fn new(state: Arc<Mutex<Option<CounterState>>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Mempool for MempoolConnection {
    async fn check_tx(&self, check_tx_request: CheckTxRequest) -> Result<CheckTxResponse> {
        let new_counter = parse_bytes_to_counter(&check_tx_request.tx)?;

        let state_lock = self.state.lock().unwrap();
        let state = state_lock.as_ref().unwrap();

        if state.counter + 1 != new_counter {
            Err(Error {
                code: 2,
                codespace: "Validation error".to_owned(),
                log: "Only consecutive integers are allowed".to_owned(),
                info: "Numbers to counter app should be supplied in increasing order of consecutive integers staring from 1".to_owned(),
            })
        } else {
            Ok(Default::default())
        }
    }
}

pub struct InfoConnection {
    state: Arc<Mutex<CounterState>>,
}

impl InfoConnection {
    pub fn new(state: Arc<Mutex<CounterState>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Info for InfoConnection {
    async fn info(&self, _info_request: InfoRequest) -> InfoResponse {
        let state = self.state.lock().unwrap();

        InfoResponse {
            data: Default::default(),
            version: Default::default(),
            app_version: Default::default(),
            last_block_height: (*state).block_height,
            last_block_app_hash: (*state).app_hash.clone(),
        }
    }
}

fn parse_bytes_to_counter(bytes: &[u8]) -> Result<u64> {
    if bytes.len() != 8 {
        return Err(Error {
            code: 1,
            codespace: "Parsing error".to_owned(),
            log: "Transaction should be 8 bytes long".to_owned(),
            info: "Transaction is big-endian encoding of 64-bit integer".to_owned(),
        });
    }

    let mut counter_bytes = [0; 8];
    counter_bytes.copy_from_slice(bytes);

    Ok(u64::from_be_bytes(counter_bytes))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let committed_state: Arc<Mutex<CounterState>> = Default::default();
    let current_state: Arc<Mutex<Option<CounterState>>> = Default::default();

    let consensus = ConsensusConnection::new(committed_state.clone(), current_state.clone());
    let mempool = MempoolConnection::new(current_state.clone());
    let info = InfoConnection::new(committed_state.clone());

    let server = Server::new(consensus, mempool, info, false);

    server
        .run("127.0.0.1:26658".parse::<SocketAddr>().unwrap())
        .await
}
