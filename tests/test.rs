use apollo_client::ClientConfig;

#[test]
fn test_client_request() {
    let client_config = ClientConfig {
        app_id: "SampleApp",
        ..Default::default()
    };
    dbg!(client_config);
}