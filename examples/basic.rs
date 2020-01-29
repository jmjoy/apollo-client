use apollo_client::{ApolloClientResult, Client, ClientConfig, Configuration, IpValue};

#[async_std::main]
async fn main() -> ApolloClientResult<()> {
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
    let configuration: ApolloClientResult<Configuration<serde_json::Value>> =
        Client::new(client_config).request().await;
    dbg!(&configuration);

    Ok(())
}
