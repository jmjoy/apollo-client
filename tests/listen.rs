use apollo_client::{ApolloClientResult, Client, ClientConfig, Response};

mod common;

#[async_std::test]
async fn test_client_listen() -> ApolloClientResult<()> {
    common::setup();

    let client_config = ClientConfig {
        app_id: "SampleApp",
        namespace_names: vec!["application", "application.yml"],
        ..Default::default()
    };

    let result: Vec<Response> = Client::with_config(&client_config).request().await?;
    assert_eq!(result.len(), 2);
    assert_eq!(&result[0].app_id, "SampleApp");
    assert_eq!(&result[0].cluster, "default");
    assert_eq!(&result[0].namespace_name, "application");
    assert_eq!(&result[0].configurations["timeout"], "100");

    Ok(())
}
