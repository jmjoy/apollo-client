//! Apollo Open APIs apis.
//!
//! Ref: <https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform>.

pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    errors::ApolloClientResult,
    meta::{handle_url, validate_response, PerformResponse, DEFAULT_TIMEOUT},
    open::requests::PerformOpenRequest,
};
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use url::Url;

/// The builder of [OpenApiClient].
pub struct OpenApiClientBuilder {
    portal_url: Url,
    token: String,
    client_builder: ClientBuilder,
}

impl OpenApiClientBuilder {
    pub fn new(portal_url: Url, token: impl ToString) -> ApolloClientResult<Self> {
        let mut builder = Self {
            portal_url,
            token: token.to_string(),
            client_builder: Default::default(),
        };
        let default_headers = builder.default_headers()?;
        builder.client_builder = builder
            .client_builder
            .timeout(DEFAULT_TIMEOUT)
            .default_headers(default_headers);
        Ok(builder)
    }

    pub fn with_client_builder(mut self, f: impl FnOnce(ClientBuilder) -> ClientBuilder) -> Self {
        self.client_builder = f(self.client_builder);
        self
    }

    pub fn build(self) -> ApolloClientResult<OpenApiClient> {
        Ok(OpenApiClient {
            portal_url: self.portal_url,
            client: self.client_builder.build()?,
        })
    }

    fn default_headers(&self) -> Result<HeaderMap, http::Error> {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, HeaderValue::from_str(&self.token)?);
        Ok(map)
    }
}

/// Created by [OpenApiClientBuilder::build].
pub struct OpenApiClient {
    portal_url: Url,
    client: Client,
}

impl OpenApiClient {
    pub async fn execute<R: PerformResponse>(
        &self,
        request: impl PerformOpenRequest<Response = R>,
    ) -> ApolloClientResult<R> {
        let url = handle_url(&request, self.portal_url.clone())?;
        let mut request_builder = self.client.request(request.method(), url);
        request_builder = request.request_builder(request_builder);
        let response = request_builder.send().await?;
        let response = validate_response(response).await?;
        <R>::from_response(response).await
    }
}
