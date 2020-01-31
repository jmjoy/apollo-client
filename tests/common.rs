use std::sync::Once;
use std::time::Duration;
use futures_timer::Delay;
use std::process::exit;


use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};


static START: Once = Once::new();

pub fn setup() {
    START.call_once(|| {
        env_logger::init();
    });
}

pub fn test_timeout(dur: Duration) {
    tokio::spawn(async move {
        Delay::new(dur.clone()).await;
        log::error!("Test failed: {:?} timeout", dur);
        exit(1);
    });
}

pub fn new_mock_server(port: u16) {
    tokio::spawn(async move {
        let make_svc = make_service_fn(|_conn| {
            async { Ok::<_, Infallible>(service_fn(mock_server_handler)) }
        });

        let addr = ([127, 0, 0, 1], port).into();
        let server = Server::bind(&addr).serve(make_svc);

        log::info!("Mock server listening on http://{}", addr);
        if let Err(e) = server.await {
            log::error!("Start mock server failed: {:?}", e);
            exit(1);
        }
    });
}

async fn mock_server_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}
