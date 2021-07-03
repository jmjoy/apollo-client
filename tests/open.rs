use std::collections::HashMap;

use apollo_client::{
    errors::{ApolloClientError, ApolloResponseError},
    open::{
        meta::{OpenCreatedItem, OpenRelease},
        OpenApiClient,
        OpenApiClientBuilder, requests::{
            OpenAppRequest, OpenClusterRequest, OpenCreateItemRequest, OpenEnvClusterRequest,
            OpenNamespaceRequest, OpenPublishNamespaceRequest,
        },
    },
};
use common::setup;

mod common;

#[tokio::test]
async fn test_env_cluster_request() {
    setup();

    let client = common::create_open_client();

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

    let client = common::create_open_client();

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

    let client = common::create_open_client();

    {
        let response = client
            .execute(
                OpenClusterRequest::builder()
                    .env("DEV")
                    .app_id("SampleApp")
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(response.name, "default");
        assert_eq!(response.app_id, "SampleApp");
    }
}

#[tokio::test]
async fn test_namespace_request() {
    setup();

    let client = common::create_open_client();

    {
        let response = client
            .execute(
                OpenNamespaceRequest::builder()
                    .env("DEV")
                    .app_id("SampleApp")
                    .build(),
            )
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn test_curd_item_request() {
    setup();

    let client = common::create_open_client();

    {
        let response = client
            .execute(
                OpenCreateItemRequest::builder()
                    .env("DEV")
                    .app_id("TestApp2")
                    .namespace_name("application")
                    .item(
                        OpenCreatedItem::builder()
                            .key("timeout")
                            .value("3000")
                            .data_change_created_by("apollo")
                            .build(),
                    )
                    .build(),
            )
            .await
            .unwrap();

        assert_eq!(response.key, "timeout");
        assert_eq!(response.value, "3000");
        assert_eq!(response.comment, None);
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .execute(
                OpenCreateItemRequest::builder()
                    .env("DEV")
                    .app_id("apollo")
                    .namespace_name("application")
                    .item(
                        OpenCreatedItem::builder()
                            .key("connect_timeout")
                            .value("100")
                            .data_change_created_by("apollo")
                            .comment("connect timeout")
                            .build(),
                    )
                    .build(),
            )
            .await
            .unwrap();

        assert_eq!(response.key, "connect_timeout");
        assert_eq!(response.value, "100");
        assert_eq!(response.comment, Some("connect timeout".to_owned()));
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .execute(
                OpenCreateItemRequest::builder()
                    .env("DEV")
                    .app_id("TestApp2")
                    .namespace_name("application")
                    .item(
                        OpenCreatedItem::builder()
                            .key("some_key")
                            .value("some_value")
                            .data_change_created_by("not_exists_user")
                            .build(),
                    )
                    .build(),
            )
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
            .execute(
                OpenPublishNamespaceRequest::builder()
                    .env("DEV")
                    .app_id("TestApp2")
                    .namespace_name("application")
                    .release(
                        OpenRelease::builder()
                            .release_title("test-release")
                            .released_by("apollo")
                            .build(),
                    )
                    .build(),
            )
            .await
            .unwrap();

        assert_eq!(response.app_id, "TestApp2");
        assert_eq!(response.cluster_name, "default");
        assert_eq!(response.namespace_name, "application");
        assert_eq!(response.data_change_created_by, "apollo");
    }
}
