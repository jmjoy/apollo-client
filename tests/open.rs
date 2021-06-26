mod common;

use apollo_client::open::{
    requests::{OpenAllNamespaceRequest, OpenAppRequest, OpenEnvClusterRequest},
    OpenApiClientBuilder,
};
use common::setup;

#[tokio::test]
async fn request_open() {
    setup();

    let client = OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse().unwrap(),
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )
    .build()
    .unwrap();
    let response = client
        .execute(OpenEnvClusterRequest::new("SampleApp"))
        .await
        .unwrap();
    dbg!(response);
    // let response = client.execute(OpenAppRequest::new(vec!["IRC-ApolloClientTest"])).await.unwrap();
    // dbg!(response);

    let response = client
        .execute(OpenAllNamespaceRequest::new("DEV", "SampleApp"))
        .await
        .unwrap();
    dbg!(response);
}
