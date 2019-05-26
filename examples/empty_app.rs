extern crate abci;
extern crate env_logger;

use env_logger::Env;

// Simple example that responds with defaults to Tendermint
struct EmptyApp;

// Implement the Application and use default responses
impl abci::Application for EmptyApp {}

fn main() {
    // Use default local addr and Tendermint ABCI port
    let addr = "127.0.0.1:26658".parse().unwrap();
    // Fire it up!
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    abci::run(addr, EmptyApp);
}
