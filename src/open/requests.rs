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
    app_ids: Option<Vec<Cow<'static, str>>>,
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

#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenClusterRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
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

#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenNamespaceRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
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

#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenCreateItemRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    item: OpenCreatedItem,
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
}

impl PerformOpenRequest for OpenCreateItemRequest {}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenUpdateItemRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    #[builder(default)]
    create_if_not_exists: bool,
    item: OpenUpdateItem,
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
            self.item.key()
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
}

impl PerformOpenRequest for OpenUpdateItemRequest {}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenPublishNamespaceRequest {
    env: Cow<'static, str>,
    app_id: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    release: OpenRelease,
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
}

impl PerformOpenRequest for OpenPublishNamespaceRequest {}
