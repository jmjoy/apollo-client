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
//!         .execute(
//!             CachedFetchRequest::builder()
//!                 .app_id("SampleApp")
//!                 .namespace_name("application.json")
//!                 .ip(IpValue::HostName)
//!                 .build(),
//!         )
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
//!     let stream = client.watch(
//!         WatchRequest::builder()
//!             .app_id("SampleApp")
//!             .namespace_names([
//!                 "application.properties".into(),
//!                 "application.json".into(),
//!                 "application.yml".into(),
//!             ])
//!             .ip(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?))
//!             .build(),
//!     );
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
//!

pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    conf::{
        meta::Notification,
        requests::{FetchRequest, NotifyRequest, PerformConfRequest, WatchRequest},
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
    ///     Url::parse("http://localhost:8080").unwrap()
    /// ).unwrap();
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
    ///
    /// let mut client_builder: ApolloConfClientBuilder = todo!();
    /// client_builder = client_builder.with_client_builder(|builder| {
    ///     builder.timeout(Duration::from_secs(6))
    /// });
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
    /// Execute configuration Api request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use apollo_client::{
    ///     conf::{ApolloConfClient, meta::IpValue, requests::CachedFetchRequest},
    ///     errors::ApolloClientResult,
    /// };
    /// use ini::Properties;
    /// use std::error::Error;
    /// use url::Url;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client: ApolloConfClient = todo!();
    ///
    ///     // Request apollo cached configuration api.
    ///     let _configuration: Properties = client
    ///         .execute(
    ///             CachedFetchRequest::builder()
    ///                 .app_id("SampleApp")
    ///                 .namespace_name("application.json")
    ///                 .ip(IpValue::HostName)
    ///                 .build(),
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    pub async fn execute<R: PerformResponse>(
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
    /// panic if request field `namespace_names` is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use apollo_client::conf::{ApolloConfClient, meta::IpValue, requests::WatchRequest};
    /// use cidr_utils::cidr::IpCidr;
    /// use futures_util::{pin_mut, stream::StreamExt};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client: ApolloConfClient = todo!();
    ///
    ///     let stream = client.watch(
    ///         WatchRequest::builder()
    ///             .app_id("SampleApp")
    ///             .namespace_names([
    ///                 "application.properties".into(),
    ///                 "application.json".into(),
    ///                 "application.yml".into(),
    ///             ])
    ///             .ip(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?))
    ///             .build(),
    ///     );
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
    ///
    pub fn watch(
        self,
        request: WatchRequest,
    ) -> impl Stream<Item = ApolloClientResult<HashMap<String, ApolloClientResult<FetchResponse>>>>
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
