pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    common::{handle_url, validate_response, PerformResponse, DEFAULT_TIMEOUT},
    conf::{
        meta::Notification,
        requests::{FetchRequest, NotifyRequest, PerformConfRequest, Watch},
        responses::FetchResponse,
    },
    errors::{
        ApolloClientError, ApolloClientError::ApolloResponse, ApolloClientResult,
        ApolloResponseError::NotModified,
    },
};
use async_stream::try_stream;
use futures_core::Stream;
use futures_util::{stream, StreamExt};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
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

    pub fn watch(
        self,
        request: Watch,
    ) -> impl Stream<Item = ApolloClientResult<Vec<ApolloClientResult<FetchResponse>>>> {
        let mut global_notifications = request.create_notifications();

        try_stream! {
            // Call the notification api first with short timeout because apollo will hang up 60s
            // and return 304 when the namespace is never be notified before.
            match self
                .execute(
                    NotifyRequest::from_watch(&request, global_notifications.clone())
                        .timeout(Duration::from_secs(1)),
                )
                .await
            {
                Ok(_) => {},
                Err(ApolloClientError::Reqwest(e)) if e.is_timeout() => {},
                Err(e) => Err(e)?,
            }

            loop {
                let notifications = match self
                    .execute(NotifyRequest::from_watch(
                        &request,
                        global_notifications.clone(),
                    ))
                    .await
                {
                    Ok(notifications) => notifications,
                    Err(ApolloResponse(NotModified)) => continue,
                    Err(e) => Err(e)?,
                };
                dbg!(&notifications);

                Notification::update_notifications(&mut global_notifications, &notifications);
                let requests = Notification::create_fetch_requests(notifications, &request);
                yield self.fetch_multi(requests).await;
            }
        }
    }

    async fn fetch_multi(
        &self,
        requests: Vec<FetchRequest>,
    ) -> Vec<ApolloClientResult<FetchResponse>> {
        let futs = requests
            .into_iter()
            .map(|fetch_request| self.execute(fetch_request))
            .collect::<Vec<_>>();

        let futs_len = futs.len();
        let futs_stream = stream::iter(futs);
        let mut buffered = futs_stream.buffer_unordered(futs_len);

        let mut rs = Vec::with_capacity(futs_len);
        while let Some(item) = buffered.next().await {
            rs.push(item);
        }
        rs
    }
}
