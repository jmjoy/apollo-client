mod common;

use apollo_client::{
    conf::{
        meta::{IpValue, Notification},
        requests::{CachedFetchRequest, FetchRequest, NotifyRequest, Watch},
        responses::FetchResponse,
        ApolloConfClient, ApolloConfClientBuilder,
    },
    errors::{ApolloClientError, ApolloResponseError},
};
use common::{ensure_timeout, setup};
use futures_util::{pin_mut, stream::StreamExt};
use ini::Properties;
use std::{collections::HashMap, time::Duration};
use apollo_client::open::{OpenApiClient, OpenApiClientBuilder};
use apollo_client::open::requests::OpenCreateItemRequest;

#[tokio::test]
async fn test_cached_fetch_request() {
    setup();

    let client = new_client_via_config_service();

    {
        let properties = client
            .execute(
                CachedFetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("application")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(properties, {
            let mut props = Properties::new();
            props.insert("timeout", "100");
            props
        });
    }

    {
        let properties = client
            .execute(
                CachedFetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("application.json")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(properties, {
            let mut props = Properties::new();
            props.insert("content", r#"{"timeout": 100}"#);
            props
        });
    }

    {
        let result = client
            .execute(
                CachedFetchRequest::builder()
                    .app_id("NotExistsApp")
                    .namespace_name("application.json")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await;
        assert!(matches!(
            result,
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }

    {
        let result = client
            .execute(
                CachedFetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("notExistsNamesapce")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await;
        assert!(matches!(
            result,
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }

    {
        let result = client
            .execute(
                CachedFetchRequest::builder()
                    .app_id("TestApp1")
                    .namespace_name("application")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await;
        assert!(matches!(result, Err(ApolloClientError::EmptyConfiguration)));
    }
}

#[tokio::test]
async fn test_fetch_request() {
    setup();

    let client = new_client_via_config_service();

    {
        let response = client
            .execute(
                FetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("application.properties")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(response.app_id, "SampleApp");
        assert_eq!(response.cluster, "default");
        assert_eq!(response.namespace_name, "application.properties");
        assert_eq!(response.configurations["timeout"], "100");
    }

    {
        let response = client
            .execute(
                FetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("application.json")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await
            .unwrap();
        assert_eq!(response.app_id, "SampleApp");
        assert_eq!(response.cluster, "default");
        assert_eq!(response.namespace_name, "application.json");
        assert_eq!(response.configurations["content"], r#"{"timeout": 100}"#);
    }

    {
        let result = client
            .execute(
                FetchRequest::builder()
                    .app_id("NotExistsApp")
                    .namespace_name("application.json")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await;
        assert!(matches!(
            result,
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }

    {
        let result = client
            .execute(
                FetchRequest::builder()
                    .app_id("SampleApp")
                    .namespace_name("notExistsNamesapce")
                    .ip(IpValue::HostName)
                    .build(),
            )
            .await;
        assert!(matches!(
            result,
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }
}

#[tokio::test]
async fn test_watch_first() {
    setup();
    ensure_timeout(Duration::from_secs(10));

    {
        let client = new_client_via_config_service();
        let stream = client.watch(
            Watch::builder()
                .app_id("TestApp1")
                .namespace_names(["foo1".into(), "foo2.properties".into()])
                .ip(IpValue::HostName)
                .build(),
        );
        pin_mut!(stream);

        let responses = stream.next().await.unwrap().unwrap();

        let foo1_response = responses["foo1"].as_ref().unwrap();
        assert_eq!(foo1_response.app_id, "TestApp1");
        assert_eq!(foo1_response.namespace_name, "foo1");
        assert_eq!(foo1_response.configurations["foo1"], "bar1");
        assert_eq!(foo1_response.configurations["foo1"], "bar1");

        let foo2_response = responses["foo2.properties"].as_ref().unwrap();
        assert_eq!(foo2_response.app_id, "TestApp1");
        assert_eq!(foo2_response.namespace_name, "foo2.properties");
        assert_eq!(foo2_response.configurations["foo2"], "bar2");
    }

    {
        let client = new_client_via_config_service();
        let stream = client.watch(
            Watch::builder()
                .app_id("NotExistsApp")
                .namespace_names(["foo1".into(), "foo2.properties".into()])
                .ip(IpValue::HostName)
                .build(),
        );
        pin_mut!(stream);

        let responses = stream.next().await.unwrap().unwrap();

        assert!(matches!(
            responses["foo1"].as_ref(),
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
        assert!(matches!(
            responses["foo2.properties"].as_ref(),
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }

    {
        let client = new_client_via_config_service();
        let stream = client.watch(
            Watch::builder()
                .app_id("TestApp1")
                .namespace_names(["foo1".into(), "not_exists_namespace".into()])
                .ip(IpValue::HostName)
                .build(),
        );
        pin_mut!(stream);

        let responses = stream.next().await.unwrap().unwrap();

        assert_eq!(responses["foo1"].as_ref().unwrap().app_id, "TestApp1");
        assert_eq!(
            responses["foo1"].as_ref().unwrap().configurations["foo1"],
            "bar1"
        );
        assert!(matches!(
            responses["not_exists_namespace"].as_ref(),
            Err(ApolloClientError::ApolloResponse(
                ApolloResponseError::NotFound
            ))
        ));
    }
}

#[tokio::test]
#[cfg(feature = "open")]
async fn test_watch_changed() {
    use apollo_client::open::{meta::{OpenCreatedItem, OpenRelease}, requests::OpenPublishNamespaceRequest};
    use tokio::time::sleep;

    setup();
    ensure_timeout(Duration::from_secs(10));

    tokio::spawn(async move {
        sleep(Duration::from_secs(3)).await;

        let client = common::create_open_client();

        client.execute(OpenCreateItemRequest::builder().env("DEV").app_id("TestApp2").namespace_name("watcher").item(
            OpenCreatedItem::builder().key("a").value("1").comment("a comment").data_change_created_by("apollo").build(),
        ).build()).await.unwrap();

        client.execute(OpenPublishNamespaceRequest::builder().env("DEV").app_id("TestApp2").namespace_name("watcher").release(
            OpenRelease::builder().release_title("release a").release_comment("release a comment").released_by("apollo").build()
        ).build()).await.unwrap();
    });

    {
        let client = new_client_via_config_service();
        let stream = client.watch(
            Watch::builder()
                .app_id("TestApp2")
                .namespace_names(["watcher".into(), "watcher2.json".into()])
                .ip(IpValue::HostName)
                .build(),
        );
        pin_mut!(stream);

        let mut index = 0usize;
        while let Some(responses) = stream.next().await {
            let responses = responses.unwrap();
            match index {
                0 => {
                    let watcher_response = responses["watcher"].as_ref().unwrap();
                    assert_eq!(watcher_response.app_id, "TestApp2");
                    assert_eq!(watcher_response.cluster, "default");
                    assert_eq!(watcher_response.namespace_name, "watcher");
                    assert_eq!(watcher_response.configurations.len(), 0);

                    let watcher2_response = responses["watcher2.json"].as_ref().unwrap();
                    assert_eq!(watcher2_response.app_id, "TestApp2");
                    assert_eq!(watcher2_response.cluster, "default");
                    assert_eq!(watcher2_response.namespace_name, "watcher2.json");
                    assert_eq!(watcher2_response.configurations.len(), 0);
                }
                1 => {}
                _ => break,
            }
            index += 1;
        }
    }
}

fn new_client_via_config_service() -> ApolloConfClient {
    ApolloConfClientBuilder::new_via_config_service("http://localhost:8080".parse().unwrap())
        .unwrap()
        .build()
        .unwrap()
}
