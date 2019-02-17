use Application;

use std::io;
use std::net::*;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;

use messages::abci::*;
use std::os::unix::net::UnixListener;
use stream::AbciStream;

#[derive(Debug)]
enum Address {
    Unix(String),
    Net(SocketAddr),
}

fn parse_addr(addr: &str) -> Option<Address> {
    let local = addr.to_lowercase();
    addr.find(':')
        .map(|i| match &local[0..i] {
            "tcp" => addr
                .get(i + 3..)
                .map(|s| s.parse().ok())
                .and_then(|e| e)
                .map(|s| Address::Net(s)),
            "unix" => addr.get(i + 3..).map(|s| Address::Unix(String::from(s))),
            _ => None,
        })
        .and_then(|e| e)
}

/// Creates the server and listens for connections from Tendermint
pub fn serve<A>(app: A, addr: &str) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let addr = parse_addr(addr).ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Wrong adderss",
    ))?;
    match addr {
        Address::Unix(addr) => serve_unix(app, &addr),
        Address::Net(addr) => serve_net(app, addr),
    }
}

/// Creates the unix server and listens for connections from Tendermint
pub fn serve_unix<A>(app: A, addr: &str) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let listener = UnixListener::bind(addr).unwrap();

    // Wrap the app atomically and clone for each connection.
    let app = Arc::new(Mutex::new(app));

    for new_connection in listener.incoming() {
        let app_instance = Arc::clone(&app);
        match new_connection {
            Ok(stream) => {
                println!("Got connection! {:?}", stream);
                thread::spawn(move || handle_stream(AbciStream::from(stream), &app_instance));
            }
            Err(err) => {
                // We need all 3 connections...
                panic!("Connection failed: {}", err);
            }
        }
    }
    drop(listener);
    Ok(())
}

/// Creates the TCP server and listens for connections from Tendermint
pub fn serve_net<A>(app: A, addr: SocketAddr) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let listener = TcpListener::bind(addr).unwrap();

    // Wrap the app atomically and clone for each connection.
    let app = Arc::new(Mutex::new(app));

    for new_connection in listener.incoming() {
        let app_instance = Arc::clone(&app);
        match new_connection {
            Ok(stream) => {
                println!("Got connection! {:?}", stream);
                thread::spawn(move || handle_stream(AbciStream::from(stream), &app_instance));
            }
            Err(err) => {
                // We need all 3 connections...
                panic!("Connection failed: {}", err);
            }
        }
    }
    drop(listener);
    Ok(())
}

fn handle_stream<A>(mut stream: AbciStream, app: &Arc<Mutex<A>>)
where
    A: Application + 'static + Send + Sync,
{
    loop {
        match stream.read_request() {
            Some(req) => {
                let mut guard = app.lock().unwrap();
                let a = guard.deref_mut();
                respond(&mut stream, a, &req).unwrap();
            }
            _ => break,
        }
    }
    println!("Connection closed on {:?}", stream);
}

fn respond<A>(stream: &mut AbciStream, app: &mut A, request: &Request) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let mut response = Response::new();
    match request.value {
        // Info
        Some(Request_oneof_value::info(ref r)) => response.set_info(app.info(r)),
        // Init chain
        Some(Request_oneof_value::init_chain(ref r)) => response.set_init_chain(app.init_chain(r)),
        // Set option
        Some(Request_oneof_value::set_option(ref r)) => response.set_set_option(app.set_option(r)),
        // Query
        Some(Request_oneof_value::query(ref r)) => response.set_query(app.query(r)),
        // Check tx
        Some(Request_oneof_value::check_tx(ref r)) => response.set_check_tx(app.check_tx(r)),
        // Begin block
        Some(Request_oneof_value::begin_block(ref r)) => {
            response.set_begin_block(app.begin_block(r))
        }
        // Deliver Tx
        Some(Request_oneof_value::deliver_tx(ref r)) => response.set_deliver_tx(app.deliver_tx(r)),
        // End block
        Some(Request_oneof_value::end_block(ref r)) => response.set_end_block(app.end_block(r)),
        // Commit
        Some(Request_oneof_value::commit(ref r)) => response.set_commit(app.commit(r)),
        // Flush
        Some(Request_oneof_value::flush(_)) => response.set_flush(ResponseFlush::new()),
        // Echo
        Some(Request_oneof_value::echo(ref r)) => {
            let echo_msg = r.get_message().to_string();
            let mut echo = ResponseEcho::new();
            echo.set_message(echo_msg);
            response.set_echo(echo);
        }
        _ => {
            let mut re = ResponseException::new();
            re.set_error(String::from("Unrecognized request"));
            response.set_exception(re)
        }
    }

    stream.write_response(&response)?;
    Ok(())
}
