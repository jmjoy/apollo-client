use std::time::Duration;

mod common;

#[tokio::test]
async fn listen_change_0() {
    common::setup();
    common::test_timeout(Duration::from_secs(10));
    common::new_mock_server(8090);

    use isahc::ResponseExt;
    let mut response = isahc::get_async("http://localhost:8090").await.unwrap();
    let text = response.text_async().await.unwrap();
    dbg!(text);
}