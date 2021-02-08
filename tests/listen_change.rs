use apollo_client::{Client, ClientConfig};
use futures_timer::Delay;
use std::{collections::HashMap, time::Duration};

mod common;

#[tokio::test]
async fn listen_change_0() {
    common::setup();
    common::test_timeout(Duration::from_secs(10));
    let mut receiver = common::new_mock_server(8090);
    receiver.recv().await.unwrap();

    let client_config = ClientConfig {
        config_server_url: "http://localhost:8090",
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.json"],
        ..Default::default()
    };

    let mut client = Client::new(client_config);

    // Request response once.
    let responses = client.listen_and_request().await.unwrap().into_inner();
    assert_eq!(responses.len(), 2);
    assert_eq!(
        responses[0]
            .as_ref()
            .unwrap()
            .deserialize_configurations::<HashMap<String, String>>()
            .unwrap()
            .get("namespace")
            .unwrap(),
        "application"
    );
    assert_eq!(
        responses[1]
            .as_ref()
            .unwrap()
            .deserialize_configurations::<HashMap<String, String>>()
            .unwrap()
            .get("namespace")
            .unwrap(),
        "application.json"
    );

    let responses = client.listen_and_request().await.unwrap();
    assert_eq!(responses.len(), 2);
}

#[tokio::test]
async fn listen_change_1() {
    common::setup();
    common::test_timeout(Duration::from_secs(10));

    tokio::spawn(async move {
        Delay::new(Duration::from_secs(5)).await;
        let mut receiver = common::new_mock_server(8091);
        receiver.recv().await.unwrap();
    });

    let client_config = ClientConfig {
        config_server_url: "http://localhost:8091",
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.json"],
        ..Default::default()
    };

    let mut client = Client::new(client_config);

    // Request response once.
    let responses = loop {
        match client.listen_and_request().await {
            Ok(responses) => break responses,
            Err(e) => {
                log::warn!("Listen failed and sleep 1 sec: {:?}", e);
                Delay::new(Duration::from_secs(1)).await;
            }
        }
    };

    assert_eq!(responses.len(), 2);
    assert_eq!(
        responses[0]
            .as_ref()
            .unwrap()
            .deserialize_configurations::<HashMap<String, String>>()
            .unwrap()
            .get("namespace")
            .unwrap(),
        "application"
    );
    assert_eq!(
        responses[1]
            .as_ref()
            .unwrap()
            .deserialize_configurations::<HashMap<String, String>>()
            .unwrap()
            .get("namespace")
            .unwrap(),
        "application.json"
    );
}
