use apollo_client::{Client, ClientConfig, IpValue};
use futures_timer::Delay;
use std::time::Duration;

#[tokio::main]
async fn main() {
    env_logger::init();

    let client_config = ClientConfig {
        config_server_url: "http://localhost:8080",
        app_id: "SampleApp",
        cluster_name: "default",
        namespace_names: vec![
            "application.properties",
            "application.json",
            "application.yml",
        ],
        ip: Some(IpValue::HostIpRegex(r"^172\.16\.")),
        ..Default::default()
    };

    let mut client = Client::new(client_config);

    loop {
        match client.listen_and_request().await {
            Ok(config) => {
                dbg!(config);
            }
            Err(e) => {
                log::error!("Listen apollo config change failed: {:?}", e);
                Delay::new(Duration::from_secs(5)).await;
            }
        }
    }
}
