//! open api requests.

use crate::{
    errors::ApolloClientResult,
    meta::{PerformRequest, DEFAULT_CLUSTER_NAME},
    open::{
        meta::{OpenCreatedItem, OpenRelease, OpenUpdateItem},
        responses::{
            OpenAppResponse, OpenClusterResponse, OpenEnvClusterResponse, OpenItemResponse,
            OpenNamespaceResponse, OpenPublishResponse,
        },
    },
};
use http::Method;
use reqwest::RequestBuilder;
use std::borrow::Cow;

const OPEN_API_PREFIX: &'static str = "openapi/v1";

/// Request executed by [crate::open::OpenApiClient::execute];
pub(crate) trait PerformOpenRequest: PerformRequest {}

/// Fetch cluster and environment infos.
#[derive(Clone, Debug)]
pub struct OpenEnvClusterRequest {
    pub app_id: String,
}

impl Default for OpenEnvClusterRequest {
    fn default() -> Self {
        OpenEnvClusterRequest {
            app_id: "".to_string(),
        }
    }
}

impl PerformRequest for OpenEnvClusterRequest {
    type Response = Vec<OpenEnvClusterResponse>;

    fn path(&self) -> String {
        format!("{}/apps/{}/envclusters", OPEN_API_PREFIX, self.app_id)
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenEnvClusterRequest {}

/// Fetch app infos.
#[derive(Clone, Debug)]
pub struct OpenAppRequest {
    pub app_ids: Option<Vec<String>>,
}

impl Default for OpenAppRequest {
    fn default() -> Self {
        OpenAppRequest { app_ids: None }
    }
}

impl PerformRequest for OpenAppRequest {
    type Response = Vec<OpenAppResponse>;

    fn path(&self) -> String {
        format!("{}/apps", OPEN_API_PREFIX)
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        Ok(self
            .app_ids
            .as_ref()
            .map(|app_ids| {
                vec![(
                    "appIds".into(),
                    app_ids
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&str>>()
                        .join(",")
                        .into(),
                )]
            })
            .unwrap_or_default())
    }
}

impl PerformOpenRequest for OpenAppRequest {}

/// Fetch cluster infos.
#[derive(Clone, Debug)]
pub struct OpenClusterRequest {
    pub env: String,
    pub app_id: String,
    pub cluster_name: String,
}

impl Default for OpenClusterRequest {
    fn default() -> Self {
        OpenClusterRequest {
            env: "".to_string(),
            app_id: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
        }
    }
}

impl PerformRequest for OpenClusterRequest {
    type Response = OpenClusterResponse;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}",
            OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name
        )
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenClusterRequest {}

/// Fetch namespace info.
#[derive(Clone, Debug)]
pub struct OpenNamespaceRequest {
    pub env: String,
    pub app_id: String,
    pub cluster_name: String,
}

impl Default for OpenNamespaceRequest {
    fn default() -> Self {
        OpenNamespaceRequest {
            env: "".to_string(),
            app_id: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
        }
    }
}

impl PerformRequest for OpenNamespaceRequest {
    type Response = Vec<OpenNamespaceResponse>;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces",
            OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name
        )
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenNamespaceRequest {}

/// Create configuration item.
#[derive(Debug, Clone)]
pub struct OpenCreateItemRequest {
    pub env: String,
    pub app_id: String,
    pub namespace_name: String,
    pub cluster_name: String,
    pub item: OpenCreatedItem,
}

impl Default for OpenCreateItemRequest {
    fn default() -> Self {
        OpenCreateItemRequest {
            env: "".to_string(),
            app_id: "".to_string(),
            namespace_name: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            item: Default::default(),
        }
    }
}

impl PerformRequest for OpenCreateItemRequest {
    type Response = OpenItemResponse;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces/{}/items",
            OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name, self.namespace_name
        )
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.json(&self.item)
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenCreateItemRequest {}

/// Update configuration item.
#[derive(Debug, Clone)]
pub struct OpenUpdateItemRequest {
    pub env: String,
    pub app_id: String,
    pub namespace_name: String,
    pub cluster_name: String,
    pub create_if_not_exists: bool,
    pub item: OpenUpdateItem,
}

impl Default for OpenUpdateItemRequest {
    fn default() -> Self {
        OpenUpdateItemRequest {
            env: "".to_string(),
            app_id: "".to_string(),
            namespace_name: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            create_if_not_exists: false,
            item: Default::default(),
        }
    }
}

impl PerformRequest for OpenUpdateItemRequest {
    type Response = ();

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces/{}/items/{}",
            OPEN_API_PREFIX,
            self.env,
            self.app_id,
            self.cluster_name,
            self.namespace_name,
            self.item.key,
        )
    }

    fn method(&self) -> Method {
        Method::PUT
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        let mut queries = vec![];
        if self.create_if_not_exists {
            queries.push(("createIfNotExists".into(), "true".into()));
        }
        Ok(queries)
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.json(&self.item)
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenUpdateItemRequest {}

/// Publish a namespace.
#[derive(Debug, Clone)]
pub struct OpenPublishNamespaceRequest {
    pub env: String,
    pub app_id: String,
    pub namespace_name: String,
    pub cluster_name: String,
    pub release: OpenRelease,
}

impl Default for OpenPublishNamespaceRequest {
    fn default() -> Self {
        OpenPublishNamespaceRequest {
            env: "".to_string(),
            app_id: "".to_string(),
            namespace_name: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            release: Default::default(),
        }
    }
}

impl PerformRequest for OpenPublishNamespaceRequest {
    type Response = OpenPublishResponse;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces/{}/releases",
            OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name, self.namespace_name
        )
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.json(&self.release)
    }

    fn app_id(&self) -> Option<&str> {
        Some(&self.app_id)
    }
}

impl PerformOpenRequest for OpenPublishNamespaceRequest {}
