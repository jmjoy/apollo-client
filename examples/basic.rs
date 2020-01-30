use apollo_client::{Client, ClientConfig, ClientResult, Configuration, IpValue};

#[async_std::main]
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
    let configuration: ClientResult<Configuration<serde_json::Value>> =
        Client::new(client_config).request().await;
    dbg!(&configuration);

    Ok(())
}
