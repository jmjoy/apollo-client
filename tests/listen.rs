use apollo_client::{Client, ClientConfig, ClientResult, Response};

use std::time::Duration;

mod common;

#[tokio::test]
/// Test the situation where just one namespace and the type is `.properties`.
async fn test_client_listen_0() -> ClientResult<()> {
    common::setup();
    common::ensure_timeout(Duration::from_secs(5));

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application"],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config)
        .listen_and_request()
        .await?
        .into_vec_response()
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}

#[tokio::test]
async fn test_client_listen_1() -> ClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.yml"],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config)
        .listen_and_request()
        .await?
        .into_vec_response()
        .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}

#[tokio::test]
async fn test_client_listen_2() -> ClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp".to_string(),
        namespace_names: vec!["application".to_string(), "application.yml".to_string()],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config)
        .listen_and_request_with_extras_query(Some(&[("noAudit", "1")]))
        .await?
        .into_vec_response()?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}
