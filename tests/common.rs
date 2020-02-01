use futures_timer::Delay;
use std::process::exit;
use std::sync::{Arc, Once};
use std::time::Duration;

use std::convert::Infallible;

use apollo_client::NamespaceKind;
use futures::try_join;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use indexmap::map::IndexMap;
use quick_error::quick_error;
use std::collections::HashMap;
use tokio::sync::mpsc;

use std::sync::atomic::{AtomicI32, Ordering};

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

quick_error! {
    #[derive(Debug)]
    pub enum MockServerError {
        Hyper(err: hyper::error::Error) {
            from()
            description("hyper error")
            display("Hyper error: {}", err)
            cause(err)
        }
        MpscSend(err: mpsc::error::SendError<()>) {
            from()
            description("mpsc send error")
            display("Mpsc send error: {}", err)
            cause(err)
        }
    }
}

pub fn new_mock_server(port: u16) -> mpsc::Receiver<()> {
    let (mut sender, receiver) = mpsc::channel(1);

    tokio::spawn(async move {
        let index = Arc::new(AtomicI32::new(0));
        let make_svc = make_service_fn(move |_conn| {
            let index = index.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |request| {
                    mock_server_handler(request, index.clone())
                }))
            }
        });

        let addr = ([127, 0, 0, 1], port).into();
        let server = Server::bind(&addr).serve(make_svc);

        log::info!("Mock server listening on http://{}", addr);

        let send_fut = async move {
            sender
                .send(())
                .await
                .map_err(|err| MockServerError::MpscSend(err))
        };
        let server_fut = async move { server.await.map_err(|err| MockServerError::Hyper(err)) };

        if let Err(e) = try_join!(send_fut, server_fut) {
            log::error!("Start mock server failed: {:?}", e);
            exit(1);
        }
    });

    receiver
}

async fn mock_server_handler(
    request: Request<Body>,
    index: Arc<AtomicI32>,
) -> Result<Response<Body>, Infallible> {
    let uri = request.uri();
    let path = uri.path();

    if path.starts_with("/configs/") {
        let split = path.split('/').collect::<Vec<_>>();
        let app_id = split[2].to_owned();
        let cluster = split[3].to_owned();
        let namespace_name = split[4].to_owned();
        let release_key = "00000".to_owned();

        let mut configurations = IndexMap::new();
        match NamespaceKind::infer_namespace_kind(&namespace_name) {
            NamespaceKind::Properties => {
                configurations.insert("namespace".to_owned(), namespace_name.to_owned());
            }
            NamespaceKind::Json => {
                configurations.insert(
                    "content".to_owned(),
                    format!(r#"{{"namespace":"{}"}}"#, namespace_name.to_owned()),
                );
            }
            _ => unreachable!(),
        }

        let response = apollo_client::Response {
            app_id,
            cluster,
            namespace_name,
            configurations,
            release_key,
        };
        let body = serde_json::to_string(&response).unwrap();
        Ok(Response::new(Body::from(body)))
    } else if path.starts_with("/notifications/") {
        let index = index.fetch_add(1, Ordering::SeqCst);
        let mut has_changed = false;

        let query = uri.query().unwrap();
        let query: HashMap<String, String> = serde_urlencoded::from_str(query).unwrap();
        let notifications = query.get("notifications").unwrap();
        let mut notifications: Vec<HashMap<String, serde_json::Value>> =
            serde_json::from_str(notifications).unwrap();
        for notification in &mut notifications {
            let id = notification.get_mut("notificationId").unwrap();
            if *id == -1 {
                *id = serde_json::Value::Number(1.into());
                has_changed = true;
            }
        }
        if !has_changed && index < 2 {
            Delay::new(Duration::from_secs(10)).await;
            Ok(Response::new(Body::from("")))
        } else {
            let body = serde_json::to_string(&notifications).unwrap();
            Ok(Response::new(Body::from(body)))
        }
    } else {
        unreachable!()
    }
}
