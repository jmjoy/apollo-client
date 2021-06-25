pub mod requests;

use reqwest::{ClientBuilder, Client};
use url::Url;
use crate::errors::ApolloClientResult;

enum ServerUrl {
    ConfigServer(Url),
    MetaServer(Url),
}

pub struct ApolloClientBuilder {
    server_url: ServerUrl,
    client_builder: ClientBuilder,
}

impl ApolloClientBuilder {
    pub fn new_via_config_service(config_server_url: Url) -> Self {
        Self {
            server_url: ServerUrl::ConfigServer(config_server_url),
            client_builder: Default::default(),
        }
    }

    pub fn build(self) -> ApolloClientResult<ApolloClient> {
        Ok(ApolloClient {
            server_url: self.server_url,
            client: self.client_builder.build()?,
        })
    }
}

pub struct ApolloClient {
    server_url: ServerUrl,
    client: Client,
}
