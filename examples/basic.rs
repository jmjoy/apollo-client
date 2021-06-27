use apollo_client::{
    conf::{meta::IpValue, requests::CachedFetchRequest, ApolloConfClientBuilder},
    errors::ApolloClientResult,
};
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
        .execute(CachedFetchRequest::new("SampleApp", "application.json").ip(IpValue::HostName))
        .await?;

    // Get the content of configuration.
    let content = configuration.get("content");
    dbg!(content);

    Ok(())
}
