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

    // Request config once.
    let config: Configuration<serde_json::Value> =
        Client::with_config(client_config)?.request().await?;

    dbg!(config);

    Ok(())
}
