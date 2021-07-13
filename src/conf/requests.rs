//! Configuration api requests.

use crate::{
    conf::{
        meta::{IpValue, Notification},
        responses::FetchResponse,
    },
    errors::ApolloClientResult,
    meta::{PerformRequest, DEFAULT_CLUSTER_NAME, DEFAULT_NOTIFY_TIMEOUT},
};
use ini::Properties;
use reqwest::RequestBuilder;
use std::{borrow::Cow, time::Duration};
use typed_builder::TypedBuilder;

/// Request executed by [crate::conf::ApolloConfClient::execute];
pub trait PerformConfRequest: PerformRequest {}

/// Request configuration from cache.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct CachedFetchRequest {
    app_id: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    ip: Option<IpValue>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    #[builder(default)]
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl PerformRequest for CachedFetchRequest {
    type Response = Properties;

    fn path(&self) -> String {
        format!(
            "/configfiles/{app_id}/{cluster_name}/{namespace_name}",
            app_id = self.app_id,
            cluster_name = self.cluster_name,
            namespace_name = self.namespace_name
        )
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        let mut pairs = vec![];
        if let Some(ip) = &self.ip {
            pairs.push(("ip".into(), ip.to_string().into()));
        }
        if !self.extras_queries.is_empty() {
            pairs.extend_from_slice(&self.extras_queries);
        }
        Ok(pairs)
    }
}

impl PerformConfRequest for CachedFetchRequest {}

/// Request configuration without cache.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct FetchRequest {
    app_id: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    ip: Option<IpValue>,
    #[builder(default, setter(strip_option))]
    release_key: Option<String>,
    #[builder(default)]
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl FetchRequest {
    pub(crate) fn namespace_name(&self) -> String {
        self.namespace_name.to_string()
    }

    pub(crate) fn from_watch(
        watch: &WatchRequest,
        namespace_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            app_id: watch.app_id.clone(),
            cluster_name: watch.cluster_name.clone(),
            namespace_name: namespace_name.into(),
            ip: watch.ip.clone(),
            release_key: None,
            extras_queries: watch.extras_queries.clone(),
        }
    }
}

impl PerformRequest for FetchRequest {
    type Response = FetchResponse;

    fn path(&self) -> String {
        format!(
            "/configs/{app_id}/{cluster_name}/{namespace_name}",
            app_id = self.app_id,
            cluster_name = self.cluster_name,
            namespace_name = self.namespace_name
        )
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        let mut pairs = vec![];
        if let Some(ip) = &self.ip {
            pairs.push(("ip".into(), ip.to_string().into()));
        }
        if let Some(release_key) = &self.release_key {
            pairs.push(("releaseKey".into(), release_key.clone().into()));
        }
        if !self.extras_queries.is_empty() {
            pairs.extend_from_slice(&self.extras_queries);
        }
        Ok(pairs)
    }
}

impl PerformConfRequest for FetchRequest {}

/// Listen apollo notification api.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct NotifyRequest {
    app_id: Cow<'static, str>,
    notifications: Vec<Notification>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    #[builder(default_code = "DEFAULT_NOTIFY_TIMEOUT")]
    timeout: Duration,
}

impl NotifyRequest {
    pub(crate) fn from_watch(
        watch: &WatchRequest,
        notifications: Vec<Notification>,
        timeout: Duration,
    ) -> Self {
        Self {
            app_id: watch.app_id.clone(),
            cluster_name: watch.cluster_name.clone(),
            notifications,
            timeout,
        }
    }
}

impl PerformRequest for NotifyRequest {
    type Response = Vec<Notification>;

    fn path(&self) -> String {
        "/notifications/v2".to_string()
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        let notifications = &self.notifications;
        // let notifications = &self
        //     .notifications
        //     .iter()
        //     .map(|n| Notification::canonicalize(n.clone()))
        //     .collect::<Vec<_>>();

        Ok(vec![
            ("appId".into(), self.app_id.clone()),
            ("cluster".into(), self.cluster_name.clone()),
            (
                "notifications".into(),
                serde_json::to_string(notifications)?.into(),
            ),
        ])
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.timeout(self.timeout)
    }
}

impl PerformConfRequest for NotifyRequest {}

/// watch multi namespaces.
///
/// Can only be used in [crate::conf::ApolloConfClient::watch].
#[derive(Clone, Debug, TypedBuilder)]
#[builder(doc, field_defaults(setter(into)))]
pub struct WatchRequest {
    app_id: Cow<'static, str>,
    namespace_names: Vec<Cow<'static, str>>,
    #[builder(default_code = "DEFAULT_CLUSTER_NAME.into()")]
    cluster_name: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    ip: Option<IpValue>,
    #[builder(default)]
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl WatchRequest {
    pub(crate) fn create_notifications(&self) -> Vec<Notification> {
        self.namespace_names
            .iter()
            .map(|namespace| {
                Notification::builder()
                    .namespace_name(namespace.clone())
                    .build()
            })
            .collect()
    }
}
