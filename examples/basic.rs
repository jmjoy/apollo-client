use apollo_client::conf::{meta::IpValue, requests::CachedFetchRequest, ApolloConfClientBuilder};
use ini::Properties;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create configuration client.
    let client =
        ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
            .build()?;

    // Request apollo cached configuration api.
    let configuration: Properties = client
        .cached_fetch(CachedFetchRequest {
            app_id: "SampleApp".to_string(),
            namespace_name: "application.json".to_string(),
            ip: Some(IpValue::HostName),
            ..Default::default()
        })
        .await?;

    // Get the content of configuration.
    let content = configuration.get("content");
    dbg!(content);

    Ok(())
}
