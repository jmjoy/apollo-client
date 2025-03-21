use apollo_client::{
    errors::ApolloClientError,
    open::{
        meta::{OpenCreatedItem, OpenRelease},
        requests::{
            OpenAppRequest, OpenClusterRequest, OpenCreateItemRequest, OpenEnvClusterRequest,
            OpenNamespaceRequest, OpenPublishNamespaceRequest,
        },
    },
};
use common::setup;
use http::StatusCode;
use std::collections::HashMap;

mod common;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_env_cluster_request() {
    setup();

    let client = common::create_open_client();

    {
        let response = client
            .env_cluster(OpenEnvClusterRequest {
                app_id: "SampleApp".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].env, "DEV");
        assert_eq!(response[0].clusters, ["default"]);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_app_request() {
    setup();

    let client = common::create_open_client();

    {
        let responses = client
            .app(OpenAppRequest {
                app_ids: Some(vec!["NotExists".into()]),
            })
            .await
            .unwrap();
        assert_eq!(responses.len(), 0);
    }

    {
        let responses = client
            .app(OpenAppRequest {
                app_ids: Some(vec!["SampleApp".into(), "TestApp1".into()]),
            })
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
        let responses = client.app(OpenAppRequest::default()).await.unwrap();
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

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_cluster_request() {
    setup();

    let client = common::create_open_client();

    {
        let response = client
            .cluster(OpenClusterRequest {
                env: "DEV".to_string(),
                app_id: "SampleApp".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(response.name, "default");
        assert_eq!(response.app_id, "SampleApp");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_namespace_request() {
    setup();

    let client = common::create_open_client();

    {
        let _response = client
            .namespace(OpenNamespaceRequest {
                env: "DEV".to_string(),
                app_id: "SampleApp".to_string(),
                ..Default::default()
            })
            .await
            .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_curd_item_request() {
    setup();

    let client = common::create_open_client();

    {
        let response = client
            .create_item(OpenCreateItemRequest {
                env: "DEV".to_string(),
                app_id: "TestApp2".to_string(),
                namespace_name: "application".to_string(),
                item: OpenCreatedItem {
                    key: "timeout".to_string(),
                    value: "3000".to_string(),
                    comment: None,
                    data_change_created_by: "apollo".to_string(),
                },
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(response.key, "timeout");
        assert_eq!(response.value, "3000");
        assert_eq!(response.comment, None);
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .create_item(OpenCreateItemRequest {
                env: "DEV".to_string(),
                app_id: "TestApp2".to_string(),
                namespace_name: "application".to_string(),
                item: OpenCreatedItem {
                    key: "connect_timeout".to_string(),
                    value: "100".to_string(),
                    comment: Some("connect timeout".to_owned()),
                    data_change_created_by: "apollo".to_string(),
                },
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(response.key, "connect_timeout");
        assert_eq!(response.value, "100");
        assert_eq!(response.comment, Some("connect timeout".to_owned()));
        assert_eq!(response.data_change_created_by, "apollo");
    }

    {
        let response = client
            .create_item(OpenCreateItemRequest {
                env: "DEV".to_string(),
                app_id: "TestApp2".to_string(),
                namespace_name: "application".to_string(),
                item: OpenCreatedItem {
                    key: "some_key".to_string(),
                    value: "some_value".to_string(),
                    comment: None,
                    data_change_created_by: "not_exists_user".to_string(),
                },
                ..Default::default()
            })
            .await;

        assert!(matches!(
            response,
            Err(ApolloClientError::ApolloResponse(e))  if e.status == StatusCode::BAD_REQUEST
        ));
    }

    {
        let response = client
            .publish_namespace(OpenPublishNamespaceRequest {
                env: "DEV".to_string(),
                app_id: "TestApp2".to_string(),
                namespace_name: "application".to_string(),
                release: OpenRelease {
                    release_title: "test-release".to_string(),
                    release_comment: None,
                    released_by: "apollo".to_string(),
                },
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(response.app_id, "TestApp2");
        assert_eq!(response.cluster_name, "default");
        assert_eq!(response.namespace_name, "application");
        assert_eq!(response.data_change_created_by, "apollo");
    }
}
