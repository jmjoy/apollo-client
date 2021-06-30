use crate::{
    errors::ApolloClientResult,
    meta::{PerformRequest, DEFAULT_CLUSTER_NAME},
    open::{
        meta::{Namespace, OpenCreatedItem, Release},
        responses::{
            OpenAppResponse, OpenClusterResponse, OpenEnvClusterResponse, OpenItemResponse,
            OpenNamespaceResponse, OpenPublishResponse,
        },
    },
};
use http::Method;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

const OPEN_API_PREFIX: &'static str = "/openapi/v1";

pub trait PerformOpenRequest: PerformRequest {}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenEnvClusterRequest {
    app_id: Cow<'static, str>,
}

impl PerformRequest for OpenEnvClusterRequest {
    type Response = Vec<OpenEnvClusterResponse>;

    fn path(&self) -> String {
        format!("{}/apps/{}/envclusters", OPEN_API_PREFIX, self.app_id)
    }
}

impl PerformOpenRequest for OpenEnvClusterRequest {}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenAppRequest {
    #[builder(default, setter(strip_option))]
    app_ids: Option<Vec<String>>,
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
            .map(|app_ids| vec![("appIds".into(), app_ids.join(",").into())])
            .unwrap_or_default())
    }
}

impl PerformOpenRequest for OpenAppRequest {}

#[derive(Clone, Debug)]
pub struct OpenClusterRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
}

impl OpenClusterRequest {
    pub fn new(env: impl Into<Cow<'static, str>>, app_id: impl Into<Cow<'static, str>>) -> Self {
        Self {
            env: env.into(),
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
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
}

impl PerformOpenRequest for OpenClusterRequest {}

#[derive(Clone, Debug)]
pub struct OpenNamespaceRequest {
    env: String,
    app_id: String,
    cluster_name: Cow<'static, str>,
}

impl OpenNamespaceRequest {
    pub fn new(env: impl ToString, app_id: impl ToString) -> Self {
        Self {
            env: env.to_string(),
            app_id: app_id.to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
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
}

impl PerformOpenRequest for OpenNamespaceRequest {}

#[derive(Debug, Clone)]
pub struct OpenCreateItemRequest {
    namespace: Namespace,
    item: OpenCreatedItem,
}

impl OpenCreateItemRequest {
    pub fn new(namespace: Namespace, item: OpenCreatedItem) -> Self {
        Self { namespace, item }
    }
}

impl PerformRequest for OpenCreateItemRequest {
    type Response = OpenItemResponse;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces/{}/items",
            OPEN_API_PREFIX,
            self.namespace.env,
            self.namespace.app_id,
            self.namespace.cluster_name,
            self.namespace.namespace_name
        )
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.json(&self.item)
    }
}

impl PerformOpenRequest for OpenCreateItemRequest {}

#[derive(Debug, Clone)]
pub struct OpenUpdateItemRequest {
    namespace: Namespace,
}

impl OpenUpdateItemRequest {}

#[derive(Debug, Clone)]
pub struct OpenPublishNamespaceRequest {
    namespace: Namespace,
    release: Release,
}

impl OpenPublishNamespaceRequest {
    pub fn new(namespace: Namespace, release: Release) -> Self {
        Self { namespace, release }
    }
}

impl PerformRequest for OpenPublishNamespaceRequest {
    type Response = OpenPublishResponse;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces/{}/releases",
            OPEN_API_PREFIX,
            self.namespace.env,
            self.namespace.app_id,
            self.namespace.cluster_name,
            self.namespace.namespace_name
        )
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.json(&self.release)
    }
}

impl PerformOpenRequest for OpenPublishNamespaceRequest {}
