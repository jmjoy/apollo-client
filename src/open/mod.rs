//! Apollo Open APIs Client.
//!
//! Ref: <https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform>.

pub mod requests;
pub mod responses;

pub use crate::errors::{ApolloClientResult, ApolloClientError};
use reqwest::{IntoUrl, ClientBuilder, Client, Response};
use std::time::Duration;
use http::{HeaderMap, HeaderValue, StatusCode};
use http::header::AUTHORIZATION;
use url::Url;
use std::convert::TryInto;
use crate::requests::PerformRequest;
use crate::errors::ApolloClientError::UrlCannotBeABase;
use serde::de::DeserializeOwned;
use crate::errors::ApolloResponseError;
use crate::open::requests::PerformOpenRequest;
use std::borrow::Cow;

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
            client: self.client_builder.default_headers(default_headers).build()?,
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
    pub async fn execute<R: DeserializeOwned>(&self, request: impl PerformOpenRequest<Response = R>) -> ApolloClientResult<R> {
        let url = self.handle_url(&request.path(), &request.query())?;
        let response = self.client.request(request.method(), url).send().await?;
        Self::validate_response(&response)?;
        let content = response.bytes().await?;
        Ok(serde_json::from_slice(&content)?)
    }

    fn handle_url(&self, path: &str, query: &[(Cow<'static, str>, Cow<'static, str>)]) -> ApolloClientResult<Url> {
        let mut url = self.portal_url.clone();
        url.path_segments_mut().map_err(|_| UrlCannotBeABase)?.extend(path.split('/'));
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        Ok(url)
    }

    fn validate_response(response: &Response) -> ApolloClientResult<()> {
        match ApolloResponseError::from_status_code(response.status()) {
            Some(e) => Err(e.into()),
            None => Ok(()),
        }
    }
}
