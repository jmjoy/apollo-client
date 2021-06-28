use crate::{
    common::{PerformRequest, PerformResponse, DEFAULT_CLUSTER_NAME},
    errors::ApolloClientResult,
    open::{
        meta::{Namespace, OpenCreatedItem, Release},
        responses::{
            OpenAppResponse, OpenEnvClusterResponse, OpenItemResponse, OpenNamespaceResponse,
            OpenPublishResponse,
        },
    },
};
use http::Method;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, prelude::rust_2015::Result::Ok};

const OPEN_API_PREFIX: &'static str = "/openapi/v1";

pub trait PerformOpenRequest: PerformRequest {}

#[derive(Clone, Debug)]
pub struct OpenEnvClusterRequest {
    app_id: String,
}

impl OpenEnvClusterRequest {
    pub fn new(app_id: impl ToString) -> Self {
        Self {
            app_id: app_id.to_string(),
        }
    }
}

impl PerformRequest for OpenEnvClusterRequest {
    type Response = Vec<OpenEnvClusterResponse>;

    fn path(&self) -> String {
        format!("{}/apps/{}/envclusters", OPEN_API_PREFIX, self.app_id)
    }
}

impl PerformOpenRequest for OpenEnvClusterRequest {}

#[derive(Clone, Debug)]
pub struct OpenAppRequest {
    app_ids: Option<Vec<String>>,
}

impl OpenAppRequest {
    pub fn new<S: ToString>(app_ids: impl Into<Vec<S>>) -> Self {
        Self {
            app_ids: Some(app_ids.into().into_iter().map(|s| s.to_string()).collect()),
        }
    }

    pub fn all() -> Self {
        Self { app_ids: None }
    }
}

impl PerformRequest for OpenAppRequest {
    type Response = Vec<OpenAppResponse>;

    fn path(&self) -> String {
        format!("{}/apps", OPEN_API_PREFIX)
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
        Ok(match &self.app_ids {
            Some(app_ids) => vec![("appIds".into(), app_ids.join(",").into())],
            None => vec![],
        })
    }
}

impl PerformOpenRequest for OpenAppRequest {}

#[derive(Clone, Debug)]
pub struct OpenNamespaceRequest {}

#[derive(Clone, Debug)]
pub struct OpenAllNamespaceRequest {
    env: String,
    app_id: String,
    cluster_name: Cow<'static, str>,
}

impl OpenAllNamespaceRequest {
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

impl PerformRequest for OpenAllNamespaceRequest {
    type Response = Vec<OpenNamespaceResponse>;

    fn path(&self) -> String {
        format!(
            "{}/envs/{}/apps/{}/clusters/{}/namespaces",
            OPEN_API_PREFIX, self.env, self.app_id, self.cluster_name
        )
    }
}

impl PerformOpenRequest for OpenAllNamespaceRequest {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    item: OpenCreatedItem,
}

impl CreateItemRequest {
    pub fn new(
        env: impl Into<Cow<'static, str>>,
        app_id: impl Into<Cow<'static, str>>,
        namespace_name: impl Into<Cow<'static, str>>,
        item: OpenCreatedItem,
    ) -> Self {
        Self {
            env: env.into(),
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            namespace_name: namespace_name.into(),
            item,
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }
}

impl PerformRequest for CreateItemRequest {
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
}

impl PerformOpenRequest for CreateItemRequest {}

#[derive(Debug, Clone)]
pub struct PublishNamespaceRequest<'a> {
    namespace: Namespace<'a>,
    release: Release<'a>,
}

impl<'a> PublishNamespaceRequest<'a> {
    pub fn new(namespace: Namespace<'a>, release: Release<'a>) -> Self {
        Self { namespace, release }
    }
}

impl PerformRequest for PublishNamespaceRequest<'_> {
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
