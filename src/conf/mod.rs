pub mod requests;
pub mod responses;

use crate::{
    common::{handle_url, validate_response, PerformResponse},
    conf::requests::PerformConfRequest,
    errors::{ApolloClientError::UrlCannotBeABase, ApolloClientResult, ApolloResponseError},
};
use http::header::CONTENT_TYPE;
use ini::Ini;
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, str};
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
    pub fn new_via_config_service(config_server_url: Url) -> Self {
        Self {
            server_url: ServerUrl::ConfigServer(config_server_url),
            client_builder: Default::default(),
        }
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
        let response = self.client.request(request.method(), url).send().await?;
        validate_response(&response)?;
        <R>::from_response(response).await
    }
}
