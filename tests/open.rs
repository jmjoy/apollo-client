mod common;

use apollo_client::{
    errors::{ApolloClientError, ApolloResponseError},
    open::{
        meta::{Namespace, OpenCreatedItem, Release},
        requests::{
            OpenAppRequest, OpenClusterRequest, OpenCreateItemRequest, OpenEnvClusterRequest,
            OpenNamespaceRequest, OpenPublishNamespaceRequest,
        },
        OpenApiClient, OpenApiClientBuilder,
    },
};
use common::setup;
use std::collections::HashMap;

#[tokio::test]
async fn test_env_cluster_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(OpenEnvClusterRequest::builder().app_id("SampleApp").build())
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
        let responses = client
            .execute(
                OpenAppRequest::builder()
                    .app_ids(["NotExists".into()])
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(responses.len(), 0);
    }

    {
        let responses = client
            .execute(
                OpenAppRequest::builder()
                    .app_ids(vec!["SampleApp".into(), "TestApp1".into()])
                    .build(),
            )
            .await
            .unwrap();
        let responses = responses
            .into_iter()
            .map(|response| (response.app_id.clone(), response))
            .collect::<HashMap<_, _>>();
        assert_eq!(responses.len(), 2);
        assert_eq!(responses["SampleApp"].name, "Sample App");
        assert_eq!(responses["SampleApp"].org_id, "TEST1");
        assert_eq!(responses["TestApp1"].name, "TestApp1");
        assert_eq!(responses["TestApp1"].org_id, "TEST1");
    }

    {
        let responses = client
            .execute(OpenAppRequest::builder().build())
            .await
            .unwrap();
        let responses = responses
            .into_iter()
            .map(|response| (response.app_id.clone(), response))
            .collect::<HashMap<_, _>>();
        assert_eq!(responses.len(), 3);
        assert_eq!(responses["SampleApp"].name, "Sample App");
        assert_eq!(responses["SampleApp"].org_id, "TEST1");
        assert_eq!(responses["TestApp1"].name, "TestApp1");
        assert_eq!(responses["TestApp1"].org_id, "TEST1");
        assert_eq!(responses["TestApp2"].name, "TestApp2");
        assert_eq!(responses["TestApp2"].org_id, "TEST1");
    }
}

#[tokio::test]
async fn test_cluster_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(OpenClusterRequest::new("DEV", "SampleApp"))
            .await
            .unwrap();
        assert_eq!(response.name, "default");
        assert_eq!(response.app_id, "SampleApp");
    }
}

#[tokio::test]
async fn test_namespace_request() {
    setup();

    let client = create_open_client();

    {
        let response = client
            .execute(OpenNamespaceRequest::new("DEV", "SampleApp"))
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
            .execute(OpenCreateItemRequest::new(
                Namespace::new("DEV", "TestApp2", "application"),
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
            .execute(OpenCreateItemRequest::new(
                Namespace::new("DEV", "TestApp2", "application"),
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
            .execute(OpenCreateItemRequest::new(
                Namespace::new("DEV", "TestApp2", "application"),
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

    {
        let response = client
            .execute(OpenPublishNamespaceRequest::new(
                Namespace::new("DEV", "TestApp2", "application"),
                Release::new("test-release", "apollo"),
            ))
            .await
            .unwrap();

        assert_eq!(response.app_id, "TestApp2");
        assert_eq!(response.cluster_name, "default");
        assert_eq!(response.namespace_name, "application");
        assert_eq!(response.data_change_created_by, "apollo");
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
