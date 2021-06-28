use futures_util::{pin_mut, stream::StreamExt};

use apollo_client::{
    conf::{
        meta::{IpValue, Notification},
        requests::{CachedFetchRequest, FetchRequest, NotifyRequest, Watch},
        responses::FetchResponse,
        ApolloConfClient, ApolloConfClientBuilder,
    },
    errors::{ApolloClientError, ApolloResponseError},
};
use common::setup;
use ini::Properties;
use std::collections::HashMap;

mod common;

#[tokio::test]
async fn test_cached_fetch_request() {
    setup();

    let client = new_client_via_config_service();

    {
        let properties = client
            .execute(CachedFetchRequest::new("SampleApp", "application").ip(IpValue::HostName))
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
            .execute(CachedFetchRequest::new("SampleApp", "application.json").ip(IpValue::HostName))
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
                CachedFetchRequest::new("NotExistsApp", "application.json").ip(IpValue::HostName),
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
                CachedFetchRequest::new("SampleApp", "notExistsNamesapce").ip(IpValue::HostName),
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
            .execute(CachedFetchRequest::new("TestApp1", "application").ip(IpValue::HostName))
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
            .execute(FetchRequest::new("SampleApp", "application.properties").ip(IpValue::HostName))
            .await
            .unwrap();
        assert_eq!(response.app_id, "SampleApp");
        assert_eq!(response.cluster, "default");
        assert_eq!(response.namespace_name, "application.properties");
        assert_eq!(response.configurations["timeout"], "100");
    }

    {
        let response = client
            .execute(FetchRequest::new("SampleApp", "application.json").ip(IpValue::HostName))
            .await
            .unwrap();
        assert_eq!(response.app_id, "SampleApp");
        assert_eq!(response.cluster, "default");
        assert_eq!(response.namespace_name, "application.json");
        assert_eq!(response.configurations["content"], r#"{"timeout": 100}"#);
    }

    {
        let result = client
            .execute(FetchRequest::new("NotExistsApp", "application.json").ip(IpValue::HostName))
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
            .execute(FetchRequest::new("SampleApp", "notExistsNamesapce").ip(IpValue::HostName))
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
// #[cfg(feature = "open")]
async fn test_watch() {
    setup();

    {
        let client = new_client_via_config_service();
        let stream =
            client.watch(Watch::new("TestApp1", ["foo1", "foo2.properties"]).ip(IpValue::HostName));
        pin_mut!(stream);

        let responses = stream
            .next()
            .await
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|response| {
                let response = response.unwrap();
                (response.namespace_name.clone(), response)
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(responses["foo1"].app_id, "TestApp1");
        assert_eq!(responses["foo1"].configurations["foo1"], "bar1");
        assert_eq!(responses["foo2"].app_id, "TestApp1");
        assert_eq!(responses["foo2"].configurations["foo2"], "bar2");
    }

    {
        let client = new_client_via_config_service();
        let stream = client
            .watch(Watch::new("NotExistsApp", ["foo1", "foo2.properties"]).ip(IpValue::HostName));
        pin_mut!(stream);

        let responses = stream
            .next()
            .await
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|response| {
                let response = response.unwrap();
                (response.namespace_name.clone(), response)
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(responses["foo1"].app_id, "TestApp1");
        assert_eq!(responses["foo1"].configurations["foo1"], "bar1");
        assert_eq!(responses["foo2"].app_id, "TestApp1");
        assert_eq!(responses["foo2"].configurations["foo2"], "bar2");
    }
}

fn new_client_via_config_service() -> ApolloConfClient {
    ApolloConfClientBuilder::new_via_config_service("http://localhost:8080".parse().unwrap())
        .unwrap()
        .build()
        .unwrap()
}
