use crate::api::handlers::sensor_data_response;
use crate::iota_channels_lite::channel_author::Channel;
use crate::security::keystore::KeyManager;

use hyper::service::{make_service_fn, service_fn};

use std::sync::{Arc, Mutex};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
static NOTFOUND: &[u8] = b"Not Found";

pub async fn start(
    port: u16,
    channel: Arc<Mutex<Channel>>,
    store: Arc<Mutex<KeyManager>>,
) -> Result<()> {
    let addr = ([0, 0, 0, 0], port).into();

    let service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let channel = channel.clone();
        let store = store.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                responder(req, channel.clone(), store.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn responder(
    req: Request<Body>,
    channel: Arc<Mutex<Channel>>,
    store: Arc<Mutex<KeyManager>>,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/sensor_data") => sensor_data_response(req, channel, store).await,
        (&Method::GET, "/status") => Ok(Response::new(Body::from("OK"))),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}
