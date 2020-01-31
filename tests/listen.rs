use apollo_client::{Client, ClientConfig, ClientResult, Response};

mod common;

#[async_std::test]
async fn test_client_listen() -> ClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.yml"],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config).listen_and_request().await?.into_vec_response()?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}

#[async_std::test]
async fn test_client_listen_2() -> ClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp".to_string(),
        namespace_names: vec!["application".to_string(), "application.yml".to_string()],
        ..Default::default()
    };

    let result: Vec<Response> = Client::new(client_config)
        .listen_and_request_with_extras_query(Some(&[("noAudit", "1")]))
        .await?.into_vec_response()?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}
