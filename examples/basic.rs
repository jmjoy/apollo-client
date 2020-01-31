use apollo_client::{Client, ClientConfig, ClientResult, IpValue};

#[tokio::main]
async fn main() -> ClientResult<()> {
    env_logger::init();

    let client_config = ClientConfig {
        config_server_url: "http://localhost:8080",
        app_id: "SampleApp",
        cluster_name: "default",
        namespace_names: vec!["application.json"],
        ip: Some(IpValue::HostName),
        ..Default::default()
    };

    // Request response once.
    let responses = Client::new(client_config).request().await?;
    dbg!(&responses);

    let configuration = responses
        .into_first()?
        .deserialize_configurations::<serde_json::Value>()?;
    dbg!(&configuration);

    Ok(())
}
