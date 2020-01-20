use apollo_client::{Client, ClientConfig, Configuration, IpValue};
use async_std::sync::channel;
use async_std::task;

#[async_std::main]
async fn main() {
    env_logger::init();

    let (sender, receiver) = channel(1);

    // New a task to listen config changes.
    task::spawn(async move {
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

        let mut client = Client::with_config(client_config).unwrap();

        loop {
            match client
                .listen_and_request::<Configuration<serde_json::Value>>()
                .await
            {
                Ok(config) => sender.send(config).await,
                Err(e) => log::error!("Listen apollo config change failed: {:?}", e),
            }
        }
    });

    // Do your job here.
    loop {
        let config = receiver.recv().await;
        dbg!(config);
    }
}
