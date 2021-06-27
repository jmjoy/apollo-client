use futures_util::{pin_mut, stream::StreamExt};

use apollo_client::conf::{
    meta::{IpValue, Notification},
    requests::{CachedFetchRequest, FetchRequest, NotifyRequest, Watch},
    ApolloConfClientBuilder,
};
use common::setup;

mod common;

#[tokio::test]
async fn test_conf() {
    setup();

    let client =
        ApolloConfClientBuilder::new_via_config_service("http://localhost:8080".parse().unwrap())
            .unwrap()
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

    let r = client
        .execute(NotifyRequest::new(
            "TestApp1",
            vec![Notification::new("application.properties")],
        ))
        .await
        .unwrap();
    dbg!(r);

    let mut stream = Box::pin(client.watch(Watch::new("TestApp1", ["foo2"])));
    while let Some(x) = stream.next().await {
        dbg!(x);
    }
}
