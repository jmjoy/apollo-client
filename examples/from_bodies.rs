use apollo_client::{ApolloClientResult, Client, ClientConfig, Configuration, Response};

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

    let _: ApolloClientResult<()> = client.request().await;

    let _: ApolloClientResult<String> = client.request().await;
    let _: ApolloClientResult<Vec<String>> = client.request().await;
    let _: Vec<ApolloClientResult<String>> = client.request().await;

    let _: ApolloClientResult<Response> = client.request().await;
    let _: ApolloClientResult<Vec<Response>> = client.request().await;
    let _: Vec<ApolloClientResult<Response>> = client.request().await;

    let _: ApolloClientResult<Configuration<serde_json::Value>> = client.request().await;
    let _: ApolloClientResult<Vec<Configuration<serde_json::Value>>> = client.request().await;
    let _: Vec<ApolloClientResult<Configuration<serde_json::Value>>> = client.request().await;
}
