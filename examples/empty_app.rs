extern crate abci;

// Simple example that responds with defaults to Tendermint
struct EmptyApp;

// Implement the Application and use default responses
impl abci::Application for EmptyApp {}

fn main() {
    // Use default local addr and Tendermint ABCI port
    let addr = "tcp://127.0.0.1:26658";
    // Fire it up!
    abci::run(addr, EmptyApp);
}
