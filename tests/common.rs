use std::{process::exit, sync::Once, time::Duration};

use tokio::time::sleep;

use apollo_client::open::{OpenApiClient, OpenApiClientBuilder};

static START: Once = Once::new();

pub fn setup() {
    START.call_once(|| {
        env_logger::init();
    });
}

#[allow(dead_code)]
pub fn ensure_timeout(dur: Duration) {
    tokio::spawn(async move {
        sleep(dur).await;
        log::error!("Test failed: {:?} timeout", dur);
        exit(1);
    });
}

#[cfg(feature = "open")]
pub fn create_open_client() -> OpenApiClient {
    OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse().unwrap(),
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )
    .unwrap()
    .build()
    .unwrap()
}
