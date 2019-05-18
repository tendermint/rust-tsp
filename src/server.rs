use std::net::SocketAddr;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use tokio;
use tokio::codec::Decoder;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use codec::abci::ABCICodec;
use futures::future::ok;
use messages::abci::*;
use Application;

/// Creates the TCP server and listens for connections from Tendermint
pub fn serve<A>(app: A, addr: SocketAddr) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let listener = TcpListener::bind(&addr).unwrap();
    let incoming = listener.incoming();
    let app = Arc::new(Mutex::new(app));
    let server = incoming
        .map_err(|err| println!("Connection failed: {}", err))
        .for_each(move |socket| {
            println!("Got connection! {:?}", socket);
            let framed = ABCICodec::new().framed(socket);
            let (_writer, reader) = framed.split();
            let app_instance = Arc::clone(&app);

            let responses = reader.map(move |request| {
                println!("Got Request! {:?}", request);
                let response = respond(&app_instance, &request);
                return response;
            });

            let writes = responses.fold(_writer, |writer, response| {
                println!("Return Response! {:?}", response);
                writer.send(response).and_then(move |writer| {
                    // workaround for ABCI protocol
                    let mut flush_response = Response::new();
                    flush_response.set_flush(ResponseFlush::new());
                    println!("Return Flush Response! {:?}", flush_response);
                    writer.send(flush_response)
                })
            });
            tokio::spawn(writes.then(|_| Ok(())))
        });
    tokio::run(server);
    Ok(())
}

fn respond<A>(app: &Arc<Mutex<A>>, request: &Request) -> Response
where
    A: Application + 'static + Send + Sync,
{
    let mut guard = app.lock().unwrap();
    let app = guard.deref_mut();

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
    response
}
