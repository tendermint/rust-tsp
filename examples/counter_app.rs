extern crate abci;
extern crate byteorder;
extern crate env_logger;

use abci::*;
use byteorder::{BigEndian, ByteOrder};
use env_logger::Env;

// Simple counter application.  Its only state is a u64 count
// We use BigEndian to serialize the data across transactions calls
struct CounterApp {
    count: u64,
}

impl CounterApp {
    fn new() -> CounterApp {
        CounterApp { count: 0 }
    }
}

// Convert incoming tx data to the proper BigEndian size. txs.len() > 8 will return 0
fn convert_tx(tx: &[u8]) -> u64 {
    if tx.len() < 8 {
        let pad = 8 - tx.len();
        let mut x = vec![0; pad];
        x.extend_from_slice(tx);
        return BigEndian::read_u64(x.as_slice());
    }
    BigEndian::read_u64(tx)
}

impl abci::Application for CounterApp {
    // Validate transactions.  Rule:  Transactions must be incremental: 1,2,3,4...
    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        // Get the Tx [u8] and convert to u64
        let c = convert_tx(req.get_tx());
        let mut resp = ResponseCheckTx::new();

        // Validation logic
        if c != self.count + 1 {
            resp.set_code(1);
            resp.set_log(String::from("Count must be incremental!"));
            return resp;
        }

        // Update state to keep state correct for next check_tx call
        self.count = c;
        resp
    }

    fn deliver_tx(&mut self, req: &RequestDeliverTx) -> ResponseDeliverTx {
        // Get the Tx [u8]
        let c = convert_tx(req.get_tx());
        // Update state
        self.count = c;
        // Return default code 0 == bueno
        ResponseDeliverTx::new()
    }

    fn commit(&mut self, _req: &RequestCommit) -> ResponseCommit {
        // Create the response
        let mut resp = ResponseCommit::new();
        // Convert count to bits
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, self.count);
        // Set data so last state is included in the block
        resp.set_data(buf.to_vec());
        resp
    }
}

fn main() {
    // Run on localhost using default Tendermint port
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    abci::run_local(CounterApp::new());
}
