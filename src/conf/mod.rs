//! Apollo configuration apis.
//!
//! Refs: <https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide>.
//!
//! # Example
//!
//! Simple fetch configuration:
//!
//! ```
//! use apollo_client::{
//!     conf::{meta::IpValue, requests::CachedFetchRequest, ApolloConfClientBuilder},
//!     errors::ApolloClientResult,
//! };
//! use ini::Properties;
//! use std::error::Error;
//! use url::Url;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::init();
//!
//!     // Create configuration client.
//!     let client =
//!         ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
//!             .build()?;
//!
//!     // Request apollo cached configuration api.
//!     let configuration: Properties = client
//!         .cached_fetch(CachedFetchRequest {
//!             app_id: "SampleApp".to_string(),
//!             namespace_name: "application.json".to_string(),
//!             ip: Some(IpValue::HostName),
//!             ..Default::default()
//!         })
//!         .await?;
//!
//!     // Get the content of configuration.
//!     let content = configuration.get("content");
//!     dbg!(content);
//!
//!     Ok(())
//! }
//! ```
//!
//! Watch configuration and fetch when changed:
//!
//! ```no_run
//! use apollo_client::conf::{meta::IpValue, requests::WatchRequest, ApolloConfClientBuilder};
//! use cidr_utils::cidr::IpCidr;
//! use futures_util::{pin_mut, stream::StreamExt};
//! use std::error::Error;
//! use url::Url;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::init();
//!
//!     // Create configuration client.
//!     let client =
//!         ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
//!             .build()?;
//!
//!     // Request apollo notification api, and fetch configuration when notified.
//!     let stream = client.watch(WatchRequest {
//!         app_id: "SampleApp".to_string(),
//!         namespace_names: vec![
//!             "application.properties".into(),
//!             "application.json".into(),
//!             "application.yml".into(),
//!         ],
//!         ip: Some(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?)),
//!         ..Default::default()
//!     });
//!
//!     pin_mut!(stream);
//!
//!     // There is a dead loop, `next()` is returned when configuration has changed.
//!     while let Some(response) = stream.next().await {
//!         let responses = response?;
//!         for response in responses {
//!             let _ = dbg!(response);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    conf::{
        meta::Notification,
        requests::{
            CachedFetchRequest, FetchRequest, NotifyRequest, PerformConfRequest, WatchRequest,
        },
        responses::FetchResponse,
    },
    errors::{ApolloClientError::ApolloResponse, ApolloClientResult},
    meta::{
        handle_url, validate_response, PerformResponse, DEFAULT_NOTIFY_TIMEOUT, DEFAULT_TIMEOUT,
    },
};
use async_stream::stream;
use futures_core::Stream;
use futures_util::{stream, StreamExt};
use http::status::StatusCode;
use ini::Properties;
use reqwest::{Client, ClientBuilder};
use std::collections::HashMap;
use url::Url;

enum ServerUrl {
    ConfigServer(Url),
    /// Todo implement fetch config via meta server.
    #[allow(dead_code)]
    MetaServer(Url),
}

/// Builder for [ApolloConfClient].
pub struct ApolloConfClientBuilder {
    server_url: ServerUrl,
    client_builder: ClientBuilder,
}

impl ApolloConfClientBuilder {
    /// Create a client request api via config service.
    ///
    /// # Example
    ///
    /// ```
    /// use apollo_client::conf::ApolloConfClientBuilder;
    /// use url::Url;
    ///
    /// let _builder = ApolloConfClientBuilder::new_via_config_service(
    ///     Url::parse("http://localhost:8080").unwrap(),
    /// )
    /// .unwrap();
    /// ```
    pub fn new_via_config_service(config_server_url: Url) -> ApolloClientResult<Self> {
        let mut builder = Self {
            server_url: ServerUrl::ConfigServer(config_server_url),
            client_builder: Default::default(),
        };
        builder.client_builder = builder.client_builder.timeout(DEFAULT_TIMEOUT);
        Ok(builder)
    }

    /// Customize inner http client.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use apollo_client::conf::ApolloConfClientBuilder;
    /// use std::time::Duration;
    /// use url::Url;
    ///
    /// ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080").unwrap())
    ///     .unwrap()
    ///     .with_client_builder(|builder| builder.timeout(Duration::from_secs(6)));
    /// ```
    pub fn with_client_builder(mut self, f: impl FnOnce(ClientBuilder) -> ClientBuilder) -> Self {
        self.client_builder = f(self.client_builder);
        self
    }

    /// Build the [ApolloConfClient].
    pub fn build(self) -> ApolloClientResult<ApolloConfClient> {
        Ok(ApolloConfClient {
            server_url: self.server_url,
            client: self.client_builder.build()?,
        })
    }
}

/// Apollo configuration apis client.
pub struct ApolloConfClient {
    server_url: ServerUrl,
    client: Client,
}

impl ApolloConfClient {
    /// 通过带缓存的Http接口从Apollo读取配置。
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide?id=_12-%e9%80%9a%e8%bf%87%e5%b8%a6%e7%bc%93%e5%ad%98%e7%9a%84http%e6%8e%a5%e5%8f%a3%e4%bb%8eapollo%e8%af%bb%e5%8f%96%e9%85%8d%e7%bd%ae)
    pub async fn cached_fetch(
        &self,
        request: CachedFetchRequest,
    ) -> ApolloClientResult<Properties> {
        self.execute(request).await
    }

    /// 通过不带缓存的Http接口从Apollo读取配置。
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide?id=_13-%e9%80%9a%e8%bf%87%e4%b8%8d%e5%b8%a6%e7%bc%93%e5%ad%98%e7%9a%84http%e6%8e%a5%e5%8f%a3%e4%bb%8eapollo%e8%af%bb%e5%8f%96%e9%85%8d%e7%bd%ae)
    pub async fn fetch(&self, request: FetchRequest) -> ApolloClientResult<FetchResponse> {
        self.execute(request).await
    }

    /// 应用感知配置更新。
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide?id=_14-%e5%ba%94%e7%94%a8%e6%84%9f%e7%9f%a5%e9%85%8d%e7%bd%ae%e6%9b%b4%e6%96%b0)
    pub async fn notify(&self, request: NotifyRequest) -> ApolloClientResult<Vec<Notification>> {
        self.execute(request).await
    }

    async fn execute<R: PerformResponse>(
        &self,
        request: impl PerformConfRequest<Response = R>,
    ) -> ApolloClientResult<R> {
        let url = match &self.server_url {
            ServerUrl::ConfigServer(url) => handle_url(&request, url.clone())?,
            ServerUrl::MetaServer(_) => todo!("unreachable here now"),
        };
        let mut request_builder = self.client.request(request.method(), url);
        request_builder = request.request_builder(request_builder);
        let response = request_builder.send().await?;
        let response = validate_response(response).await?;
        <R>::from_response(response).await
    }

    /// Watch the multi namespaces change, and fetch namespaces configuration when changed.
    ///
    /// Return the Stream implemented [futures_core::Stream], and the return value of `poll_next`
    /// will never be None (Dead Loop).
    ///
    /// The first `poll_next` will fetch all namespaces, the remained will only fetch changed
    /// namespaces.
    ///
    /// # Panic
    ///
    /// panic if `request.namespace_names` is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use apollo_client::conf::{meta::IpValue, requests::WatchRequest, ApolloConfClient};
    /// use cidr_utils::cidr::IpCidr;
    /// use futures_util::{pin_mut, stream::StreamExt};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client: ApolloConfClient = todo!();
    ///
    ///     let stream = client.watch(WatchRequest {
    ///         app_id: "SampleApp".to_string(),
    ///         namespace_names: vec![
    ///             "application.properties".into(),
    ///             "application.json".into(),
    ///             "application.yml".into(),
    ///         ],
    ///         ip: Some(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?)),
    ///         ..Default::default()
    ///     });
    ///
    ///     pin_mut!(stream);
    ///
    ///     // This is a dead loop, `next()` is returned when configuration has changed.
    ///     while let Some(response) = stream.next().await {
    ///         let _ = response?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn watch<'a>(
        &'a self,
        request: WatchRequest,
    ) -> impl Stream<Item = ApolloClientResult<HashMap<String, ApolloClientResult<FetchResponse>>>> + 'a
    {
        let mut watch_notifications = request.create_notifications();
        let mut fetch_notifications = watch_notifications.clone();
        assert_ne!(
            watch_notifications.len(),
            0,
            "watch namespaces should not be null"
        );

        stream! {
            loop {
                let requests = Notification::create_fetch_requests(fetch_notifications, &request);
                yield Ok(self.fetch_multi(requests).await);

                loop {
                    match self
                        .execute(NotifyRequest::from_watch(
                            &request,
                            watch_notifications.clone(),
                            DEFAULT_NOTIFY_TIMEOUT,
                        ))
                        .await
                    {
                        Ok(notifications) => {
                            let is_uninitialized = watch_notifications[0].is_uninitialized();
                            Notification::update_notifications(
                                &mut watch_notifications,
                                &notifications,
                            );
                            fetch_notifications = notifications;
                            if !is_uninitialized {
                                break;
                            }
                        },
                        Err(ApolloResponse(e)) if e.status == StatusCode::NOT_MODIFIED => {},
                        Err(e) => yield Err(e),
                    }
                }
            }
        }
    }

    async fn fetch_multi(
        &self,
        requests: Vec<FetchRequest>,
    ) -> HashMap<String, ApolloClientResult<FetchResponse>> {
        let executors = requests.into_iter().map(|fetch_request| async move {
            (
                fetch_request.namespace_name(),
                self.execute(fetch_request).await,
            )
        });

        let executors_len = executors.len();
        let executors_stream = stream::iter(executors);
        let mut buffered = executors_stream.buffer_unordered(executors_len);

        let mut map = HashMap::with_capacity(executors_len);
        while let Some(item) = buffered.next().await {
            map.insert(item.0, item.1);
        }
        map
    }
}
