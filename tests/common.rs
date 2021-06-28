use std::{process::exit, sync::Once, time::Duration};
use tokio::time::sleep;

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
