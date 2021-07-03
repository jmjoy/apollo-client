//! Apollo configuration apis.
//!
//! Refs: <https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide>.

pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    conf::{
        meta::Notification,
        requests::{FetchRequest, NotifyRequest, PerformConfRequest, Watch},
        responses::FetchResponse,
    },
    errors::{
        ApolloClientError, ApolloClientError::ApolloResponse, ApolloClientResult,
        ApolloResponseError::NotModified,
    },
    meta::{
        handle_url, validate_response, PerformResponse, DEFAULT_NOTIFY_TIMEOUT, DEFAULT_TIMEOUT,
    },
};
use async_stream::stream;
use futures_core::Stream;
use futures_util::{stream, StreamExt};
use reqwest::{Client, ClientBuilder};
use std::{borrow::Cow, collections::HashMap, time::Duration};
use url::Url;

enum ServerUrl {
    ConfigServer(Url),
    /// Todo implement fetch config via meta server.
    #[allow(dead_code)]
    MetaServer(Url),
}

pub struct ApolloConfClientBuilder {
    server_url: ServerUrl,
    client_builder: ClientBuilder,
}

impl ApolloConfClientBuilder {
    pub fn new_via_config_service(config_server_url: Url) -> ApolloClientResult<Self> {
        let mut builder = Self {
            server_url: ServerUrl::ConfigServer(config_server_url),
            client_builder: Default::default(),
        };
        builder.client_builder = builder.client_builder.timeout(DEFAULT_TIMEOUT);
        Ok(builder)
    }

    pub fn with_client_builder(mut self, f: impl FnOnce(ClientBuilder) -> ClientBuilder) -> Self {
        self.client_builder = f(self.client_builder);
        self
    }

    pub fn build(self) -> ApolloClientResult<ApolloConfClient> {
        Ok(ApolloConfClient {
            server_url: self.server_url,
            client: self.client_builder.build()?,
        })
    }
}

pub struct ApolloConfClient {
    server_url: ServerUrl,
    client: Client,
}

impl ApolloConfClient {
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
        validate_response(&response)?;
        <R>::from_response(response).await
    }

    /// Watch the multi namespaces change, and fetch all namespaces configuration when changed,
    ///
    /// Return the Stream implemented [futures::strem::Stream], and the return value of `poll_next` will never be None.
    ///
    pub fn watch(
        self,
        request: Watch,
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

                fetch_notifications = loop {
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
                            Notification::update_notifications(&mut watch_notifications, &notifications);
                            if is_uninitialized {
                                // Avoid to fetch request again.
                                continue;
                            } else {
                                break notifications;
                            }
                        },
                        Err(ApolloResponse(NotModified)) => continue,
                        Err(e) => yield Err(e),
                    };
                };
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
