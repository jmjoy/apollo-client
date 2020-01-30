use apollo_client::{Client, ClientConfig, ClientResult, Configuration, Response};

#[async_std::main]
async fn main() {
    env_logger::init();

    let client_config = ClientConfig {
        config_server_url: "http://localhost:8080",
        app_id: "SampleApp",
        namespace_names: vec!["application.json"],
        ..Default::default()
    };
    let client = Client::new(client_config);

    let _: ClientResult<()> = client.request().await;

    let _: ClientResult<String> = client.request().await;
    let _: ClientResult<Vec<String>> = client.request().await;
    let _: Vec<ClientResult<String>> = client.request().await;

    let _: ClientResult<Response> = client.request().await;
    let _: ClientResult<Vec<Response>> = client.request().await;
    let _: Vec<ClientResult<Response>> = client.request().await;

    let _: ClientResult<Configuration<serde_json::Value>> = client.request().await;
    let _: ClientResult<Vec<Configuration<serde_json::Value>>> = client.request().await;
    let _: Vec<ClientResult<Configuration<serde_json::Value>>> = client.request().await;
}
