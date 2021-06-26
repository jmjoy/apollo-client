mod common;

use apollo_client::conf::{
    requests::{CachedFetchRequest, FetchRequest, IpValue},
    ApolloConfClientBuilder,
};
use common::setup;

#[tokio::test]
async fn test_conf() {
    setup();

    let client =
        ApolloConfClientBuilder::new_via_config_service("http://localhost:8080".parse().unwrap())
            .build()
            .unwrap();

    let r = client
        .execute(CachedFetchRequest::new("SampleApp", "application.yml").ip(IpValue::HostName))
        .await
        .unwrap();
    dbg!(r);

    let r = client
        .execute(FetchRequest::new("SampleApp", "application.properties").ip(IpValue::HostName))
        .await
        .unwrap();
    dbg!(r);
}
