mod common;

use apollo_client::{
    errors::{ApolloClientError, ApolloResponseError},
    open::{
        meta::OpenCreatedItem,
        requests::{
            CreateItemRequest, OpenAllNamespaceRequest, OpenAppRequest, OpenEnvClusterRequest,
        },
        OpenApiClient, OpenApiClientBuilder,
    },
};
use common::setup;

#[tokio::test]
async fn test_env_cluster_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(OpenEnvClusterRequest::new("SampleApp"))
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].env, "DEV");
        assert_eq!(response[0].clusters, ["default"]);
    }
}

#[tokio::test]
async fn test_app_request() {
    setup();

    let client = create_open_client();

    {
        // let response = client.execute(OpenAppRequest::new(vec!["IRC-ApolloClientTest"])).await.unwrap();
        // dbg!(response);
    }
}

#[tokio::test]
async fn test_all_namespace_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(OpenAllNamespaceRequest::new("DEV", "SampleApp"))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn test_curd_item_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(CreateItemRequest::new(
                "DEV",
                "TestApp2",
                "application",
                OpenCreatedItem::new("timeout", "3000", "apollo"),
            ))
            .await
            .unwrap();

        assert_eq!(response.key, "timeout");
        assert_eq!(response.value, "3000");
        assert_eq!(response.comment, None);
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .execute(CreateItemRequest::new(
                "DEV",
                "TestApp2",
                "application",
                OpenCreatedItem::new("connect_timeout", "100", "apollo").comment("connect timeout"),
            ))
            .await
            .unwrap();

        assert_eq!(response.key, "connect_timeout");
        assert_eq!(response.value, "100");
        assert_eq!(response.comment, Some("connect timeout".to_owned()));
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .execute(CreateItemRequest::new(
                "DEV",
                "TestApp2",
                "application",
                OpenCreatedItem::new("some_key", "some_value", "not_exists_user"),
            ))
            .await;

        assert!(matches!(
            response,
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::BadRequest
            ))
        ));
    }
}

fn create_open_client() -> OpenApiClient {
    OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse().unwrap(),
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )
    .unwrap()
    .build()
    .unwrap()
}
