use std::net::SocketAddr;
use std::ops::DerefMut;
use std::sync::Arc;

use env_logger::Env;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::runtime;
use tokio::sync::Mutex;
use tokio_util::codec::Decoder;

use crate::codec::ABCICodec;
use crate::messages::abci::*;
use crate::Application;

async fn serve_async<A>(app: A, addr: SocketAddr)
where
    A: Application + 'static + Send + Sync,
{
    let app = Arc::new(Mutex::new(app));
    let mut listener = TcpListener::bind(&addr).await.unwrap();
    while let Some(Ok(socket)) = listener.next().await {
        let app_instance = app.clone();
        tokio::spawn(async move {
            info!("Got connection! {:?}", socket);
            let framed = ABCICodec::new().framed(socket);
            let (mut writer, mut reader) = framed.split();
            let mut mrequest = reader.next().await;
            while let Some(Ok(ref request)) = mrequest {
                debug!("Got Request! {:?}", request);
                let response = respond(&app_instance, request).await;
                debug!("Return Response! {:?}", response);
                writer.send(response).await.expect("sending back response");
                mrequest = reader.next().await;
            }
            match mrequest {
                None => {
                    panic!("connection dropped");
                }
                Some(Err(e)) => {
                    panic!("decoding error: {:?}", e);
                }
                _ => {
                    unreachable!();
                }
            }
        });
    }
}

/// Creates the TCP server and listens for connections from Tendermint
pub fn serve<A>(app: A, addr: SocketAddr) -> std::io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    env_logger::from_env(Env::default().default_filter_or("info"))
        .try_init()
        .ok();

    let mut rt = runtime::Builder::new()
        .basic_scheduler()
        .enable_io()
        .build()
        .unwrap();
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);
        std::process::exit(1);
    }));
    rt.block_on(serve_async(app, addr));
    Ok(())
}

async fn respond<A>(app: &Arc<Mutex<A>>, request: &Request) -> Response
where
    A: Application + 'static + Send + Sync,
{
    let mut guard = app.lock().await;
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
