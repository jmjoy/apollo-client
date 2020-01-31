use std::sync::Once;
use std::time::Duration;
use futures_timer::Delay;
use std::process::exit;

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

pub fn new_mock_server() {
    tokio::spawn(async move {

    });
}
