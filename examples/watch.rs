use apollo_client::conf::{meta::IpValue, requests::WatchRequest, ApolloConfClientBuilder};
use cidr_utils::cidr::IpCidr;
use futures_util::{pin_mut, stream::StreamExt};
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create configuration client.
    let client =
        ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
            .build()?;

    // Request apollo notification api, and fetch configuration when notified.
    let stream = client.watch(WatchRequest {
        app_id: "SampleApp".to_string(),
        namespace_names: vec![
            "application.properties".into(),
            "application.json".into(),
            "application.yml".into(),
        ],
        ip: Some(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?)),
        ..Default::default()
    });

    pin_mut!(stream);

    // These is a dead loop, `next()` is returned when configuration is changed.
    while let Some(response) = stream.next().await {
        let responses = response?;
        for response in responses {
            let _ = dbg!(response);
        }
    }

    Ok(())
}
