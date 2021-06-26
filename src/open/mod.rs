//! Apollo Open APIs Client.
//!
//! Ref: <https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform>.

pub mod requests;
pub mod responses;

pub use crate::errors::{ApolloClientError, ApolloClientResult};
use crate::{
    common::{handle_url, validate_response, PerformRequest, PerformResponse},
    errors::{ApolloClientError::UrlCannotBeABase, ApolloResponseError},
    open::requests::PerformOpenRequest,
};
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, ClientBuilder, IntoUrl, Response};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, convert::TryInto, time::Duration};
use url::Url;

/// The builder of [OpenApiClient].
pub struct OpenApiClientBuilder {
    portal_url: Url,
    token: String,
    client_builder: ClientBuilder,
}

impl OpenApiClientBuilder {
    pub fn new(portal_url: Url, token: impl ToString) -> Self {
        Self {
            portal_url,
            token: token.to_string(),
            client_builder: Default::default(),
        }
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.client_builder = self.client_builder.connect_timeout(timeout);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.client_builder = self.client_builder.timeout(timeout);
        self
    }

    pub fn build(self) -> ApolloClientResult<OpenApiClient> {
        let default_headers = self.default_headers()?;

        Ok(OpenApiClient {
            portal_url: self.portal_url,
            client: self
                .client_builder
                .default_headers(default_headers)
                .build()?,
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
        let response = self.client.request(request.method(), url).send().await?;
        validate_response(&response)?;
        <R>::from_response(response).await
    }
}
