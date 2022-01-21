//! Apollo Open APIs apis.
//!
//! Ref: <https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform>.
//!
//! Call open platform api to fetch app infos:
//!
//! ```
//! use apollo_client::open::{requests::OpenAppRequest, OpenApiClientBuilder};
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     env_logger::init();
//!
//!     // Create open platform api client.
//!     let client = OpenApiClientBuilder::new(
//!         "http://127.0.0.1:8070/".parse()?,
//!         "391cc4053f8cce2e452a0e6db8925bbba503f434",
//!     )?
//!     .build()?;
//!
//!     // Execute app fetching request.
//!     let responses = client
//!         .app(OpenAppRequest {
//!             app_ids: Some(vec!["SampleApp".into()]),
//!         })
//!         .await?;
//!
//!     dbg!(responses);
//!
//!     Ok(())
//! }
//! ```

pub mod meta;
pub mod requests;
pub mod responses;

use crate::{
    errors::ApolloClientResult,
    meta::{handle_url, validate_response, PerformResponse, DEFAULT_TIMEOUT},
    open::{
        requests::{
            OpenAppRequest, OpenClusterRequest, OpenCreateItemRequest, OpenEnvClusterRequest,
            OpenNamespaceRequest, OpenPublishNamespaceRequest, OpenUpdateItemRequest,
            PerformOpenRequest,
        },
        responses::{
            OpenAppResponse, OpenClusterResponse, OpenEnvClusterResponse, OpenItemResponse,
            OpenNamespaceResponse, OpenPublishResponse,
        },
    },
};
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder};
use url::Url;

/// The builder for [OpenApiClient].
pub struct OpenApiClientBuilder {
    portal_url: Url,
    token: String,
    client_builder: ClientBuilder,
}

impl OpenApiClientBuilder {
    /// Create a builder.
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

    /// Customize inner http client.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use apollo_client::open::OpenApiClientBuilder;
    /// use std::time::Duration;
    ///
    /// OpenApiClientBuilder::new(
    ///     "http://127.0.0.1:8070/".parse().unwrap(),
    ///     "391cc4053f8cce2e452a0e6db8925bbba503f434",
    /// )
    /// .unwrap()
    /// .with_client_builder(|builder| builder.timeout(Duration::from_secs(6)));
    /// ```
    pub fn with_client_builder(mut self, f: impl FnOnce(ClientBuilder) -> ClientBuilder) -> Self {
        self.client_builder = f(self.client_builder);
        self
    }

    /// Build the [OpenApiClient].
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
    /// 获取App的环境，集群信息。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_321-%e8%8e%b7%e5%8f%96app%e7%9a%84%e7%8e%af%e5%a2%83%ef%bc%8c%e9%9b%86%e7%be%a4%e4%bf%a1%e6%81%af)
    pub async fn env_cluster(
        &self,
        request: OpenEnvClusterRequest,
    ) -> ApolloClientResult<Vec<OpenEnvClusterResponse>> {
        self.execute(request).await
    }

    /// 获取App信息。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_322-%e8%8e%b7%e5%8f%96app%e4%bf%a1%e6%81%af)
    pub async fn app(&self, request: OpenAppRequest) -> ApolloClientResult<Vec<OpenAppResponse>> {
        self.execute(request).await
    }

    /// 获取集群接口。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_323-%e8%8e%b7%e5%8f%96%e9%9b%86%e7%be%a4%e6%8e%a5%e5%8f%a3)
    pub async fn cluster(
        &self,
        request: OpenClusterRequest,
    ) -> ApolloClientResult<OpenClusterResponse> {
        self.execute(request).await
    }

    /// 获取某个Namespace信息接口。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_326-%e8%8e%b7%e5%8f%96%e6%9f%90%e4%b8%aanamespace%e4%bf%a1%e6%81%af%e6%8e%a5%e5%8f%a3)
    pub async fn namespace(
        &self,
        request: OpenNamespaceRequest,
    ) -> ApolloClientResult<Vec<OpenNamespaceResponse>> {
        self.execute(request).await
    }

    /// 新增配置接口。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_3210-%e6%96%b0%e5%a2%9e%e9%85%8d%e7%bd%ae%e6%8e%a5%e5%8f%a3)
    pub async fn create_item(
        &self,
        request: OpenCreateItemRequest,
    ) -> ApolloClientResult<OpenItemResponse> {
        self.execute(request).await
    }

    /// 修改配置接口。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_3211-%e4%bf%ae%e6%94%b9%e9%85%8d%e7%bd%ae%e6%8e%a5%e5%8f%a3)
    pub async fn update_item(&self, request: OpenUpdateItemRequest) -> ApolloClientResult<()> {
        self.execute(request).await
    }

    /// 发布配置接口。
    ///
    /// [Ref](https://www.apolloconfig.com/#/zh/usage/apollo-open-api-platform?id=_3213-%e5%8f%91%e5%b8%83%e9%85%8d%e7%bd%ae%e6%8e%a5%e5%8f%a3)
    pub async fn publish_namespace(
        &self,
        request: OpenPublishNamespaceRequest,
    ) -> ApolloClientResult<OpenPublishResponse> {
        self.execute(request).await
    }

    async fn execute<R: PerformResponse>(
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
