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

/// Request executed by [crate::conf::ApolloConfClient::execute];
pub(crate) trait PerformConfRequest: PerformRequest {}

/// Request configuration from cache.
#[derive(Clone, Debug)]
pub struct CachedFetchRequest {
    pub app_id: String,
    pub namespace_name: String,
    pub ip: Option<IpValue>,
    pub cluster_name: String,
    pub extras_queries: Vec<(String, String)>,
    #[cfg(feature = "auth")]
    pub access_key: Option<String>,
}

impl Default for CachedFetchRequest {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            namespace_name: "".to_string(),
            ip: None,
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            extras_queries: vec![],
            #[cfg(feature = "auth")]
            access_key: None,
        }
    }
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

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
        let mut pairs = vec![];
        if let Some(ip) = &self.ip {
            pairs.push(("ip".into(), ip.to_string().into()));
        }
        if !self.extras_queries.is_empty() {
            pairs.extend(
                self.extras_queries
                    .iter()
                    .map(|(k, v)| (k.clone().into(), v.clone().into())),
            );
        }
        Ok(pairs)
    }

    fn app_id(&self) -> &str {
        &self.app_id
    }

    #[cfg(feature = "auth")]
    fn access_key(&self) -> Option<&str> {
        self.access_key.as_ref().map(|key| key.as_str())
    }
}

impl PerformConfRequest for CachedFetchRequest {}

/// Request configuration without cache.
#[derive(Clone, Debug)]
pub struct FetchRequest {
    pub app_id: String,
    pub namespace_name: String,
    pub cluster_name: String,
    pub ip: Option<IpValue>,
    pub release_key: Option<String>,
    pub extras_queries: Vec<(String, String)>,
    #[cfg(feature = "auth")]
    pub access_key: Option<String>,
}

impl Default for FetchRequest {
    fn default() -> Self {
        FetchRequest {
            app_id: "".to_string(),
            namespace_name: "".to_string(),
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            ip: None,
            release_key: None,
            extras_queries: vec![],
            #[cfg(feature = "auth")]
            access_key: None,
        }
    }
}

impl FetchRequest {
    pub(crate) fn namespace_name(&self) -> String {
        self.namespace_name.to_string()
    }

    pub(crate) fn from_watch(watch: &WatchRequest, namespace_name: String) -> Self {
        Self {
            app_id: watch.app_id.clone(),
            cluster_name: watch.cluster_name.clone(),
            namespace_name,
            ip: watch.ip.clone(),
            release_key: None,
            extras_queries: watch.extras_queries.clone(),
            #[cfg(feature = "auth")]
            access_key: watch.access_key.clone(),
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
            pairs.extend(
                self.extras_queries
                    .iter()
                    .map(|(k, v)| (k.clone().into(), v.clone().into())),
            );
        }
        Ok(pairs)
    }

    fn app_id(&self) -> &str {
        &self.app_id
    }

    #[cfg(feature = "auth")]
    fn access_key(&self) -> Option<&str> {
        self.access_key.as_ref().map(|key| key.as_str())
    }
}

impl PerformConfRequest for FetchRequest {}

/// Listen apollo notification api.
#[derive(Clone, Debug)]
pub struct NotifyRequest {
    pub app_id: String,
    pub notifications: Vec<Notification>,
    pub cluster_name: String,
    pub timeout: Duration,
    #[cfg(feature = "auth")]
    pub access_key: Option<String>,
}

impl Default for NotifyRequest {
    fn default() -> Self {
        NotifyRequest {
            app_id: "".to_string(),
            notifications: vec![],
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            timeout: DEFAULT_NOTIFY_TIMEOUT,
            #[cfg(feature = "auth")]
            access_key: None,
        }
    }
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
            #[cfg(feature = "auth")]
            access_key: watch.access_key.clone(),
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
        Ok(vec![
            ("appId".into(), self.app_id.clone().into()),
            ("cluster".into(), self.cluster_name.clone().into()),
            (
                "notifications".into(),
                serde_json::to_string(notifications)?.into(),
            ),
        ])
    }

    #[allow(unused_mut)]
    fn request_builder(&self, mut request_builder: RequestBuilder) -> RequestBuilder {
        //FIXME
        //see issue #15701 <https://github.com/rust-lang/rust/issues/15701>
        #[cfg(feature = "auth")]
        if true {
            request_builder = self.signature(request_builder);
        }
        request_builder.timeout(self.timeout)
    }

    fn app_id(&self) -> &str {
        &self.app_id
    }

    #[cfg(feature = "auth")]
    fn access_key(&self) -> Option<&str> {
        self.access_key.as_ref().map(|key| key.as_str())
    }
}

impl PerformConfRequest for NotifyRequest {}

/// watch multi namespaces.
///
/// Can only be used in [crate::conf::ApolloConfClient::watch].
#[derive(Clone, Debug)]
pub struct WatchRequest {
    pub app_id: String,
    pub namespace_names: Vec<String>,
    pub cluster_name: String,
    pub ip: Option<IpValue>,
    pub extras_queries: Vec<(String, String)>,
    #[cfg(feature = "auth")]
    pub access_key: Option<String>,
}

impl Default for WatchRequest {
    fn default() -> Self {
        WatchRequest {
            app_id: "".to_string(),
            namespace_names: vec![],
            cluster_name: DEFAULT_CLUSTER_NAME.to_string(),
            ip: None,
            extras_queries: vec![],
            #[cfg(feature = "auth")]
            access_key: None,
        }
    }
}

impl WatchRequest {
    pub(crate) fn create_notifications(&self) -> Vec<Notification> {
        self.namespace_names
            .iter()
            .map(|namespace| Notification {
                namespace_name: namespace.clone(),
                ..Default::default()
            })
            .collect()
    }
}
