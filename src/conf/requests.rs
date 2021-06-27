use crate::{
    common::{PerformRequest, DEFAULT_CLUSTER_NAME, DEFAULT_NOTIFY_TIMEOUT},
    conf::{
        meta::{IpValue, Notification},
        responses::FetchResponse,
    },
    errors::ApolloClientResult,
};
use ini::Properties;
use reqwest::RequestBuilder;
use std::{borrow::Cow, time::Duration};

pub trait PerformConfRequest: PerformRequest {}

#[derive(Clone, Debug)]
pub struct CachedFetchRequest {
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    ip: Option<IpValue>,
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl CachedFetchRequest {
    pub fn new(
        app_id: impl Into<Cow<'static, str>>,
        namespace_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            namespace_name: namespace_name.into(),
            ip: None,
            extras_queries: vec![],
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }

    pub fn ip(mut self, ip: IpValue) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn add_extras_query(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.extras_queries.push((name.into(), value.into()));
        self
    }

    pub fn extras_queries(
        mut self,
        extras_queries: Vec<(impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        self.extras_queries = extras_queries
            .into_iter()
            .map(|(n, v)| (n.into(), v.into()))
            .collect();
        self
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
            pairs.extend_from_slice(&self.extras_queries);
        }
        Ok(pairs)
    }
}

impl PerformConfRequest for CachedFetchRequest {}

#[derive(Clone, Debug, PartialEq)]
pub struct FetchRequest {
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
    namespace_name: Cow<'static, str>,
    ip: Option<IpValue>,
    release_key: Option<String>,
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl FetchRequest {
    pub fn new(
        app_id: impl Into<Cow<'static, str>>,
        namespace_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            namespace_name: namespace_name.into(),
            ip: None,
            release_key: None,
            extras_queries: vec![],
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }

    pub fn ip(mut self, ip: IpValue) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn release_key(mut self, release_key: String) -> Self {
        self.release_key = Some(release_key);
        self
    }

    pub fn add_extras_query(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.extras_queries.push((name.into(), value.into()));
        self
    }

    pub fn extras_queries(
        mut self,
        extras_queries: Vec<(impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        self.extras_queries = extras_queries
            .into_iter()
            .map(|(n, v)| (n.into(), v.into()))
            .collect();
        self
    }

    pub(crate) fn from_watch(watch: &Watch, namespace_name: impl Into<Cow<'static, str>>) -> Self {
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

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
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

#[derive(Clone, Debug)]
pub struct NotifyRequest {
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
    notifications: Vec<Notification>,
    timeout: Duration,
}

impl NotifyRequest {
    pub fn new(
        app_id: impl Into<Cow<'static, str>>,
        notifications: impl Into<Vec<Notification>>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            notifications: notifications.into(),
            timeout: DEFAULT_NOTIFY_TIMEOUT,
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub(crate) fn from_watch(watch: &Watch, notifications: Vec<Notification>) -> Self {
        Self {
            app_id: watch.app_id.clone(),
            cluster_name: watch.cluster_name.clone(),
            notifications,
            timeout: DEFAULT_NOTIFY_TIMEOUT,
        }
    }
}

impl PerformRequest for NotifyRequest {
    type Response = Vec<Notification>;

    fn path(&self) -> String {
        "/notifications/v2".to_string()
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
        Ok(vec![
            ("appId".into(), self.app_id.clone()),
            ("cluster".into(), self.cluster_name.clone()),
            (
                "notifications".into(),
                serde_json::to_string(&self.notifications)?.into(),
            ),
        ])
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.timeout(self.timeout)
    }
}

impl PerformConfRequest for NotifyRequest {}

#[derive(Clone, Debug)]
pub struct Watch {
    app_id: Cow<'static, str>,
    cluster_name: Cow<'static, str>,
    namespaces: Vec<Cow<'static, str>>,
    ip: Option<IpValue>,
    extras_queries: Vec<(Cow<'static, str>, Cow<'static, str>)>,
}

impl Watch {
    pub fn new<S: Into<Cow<'static, str>>>(
        app_id: impl Into<Cow<'static, str>>,
        namespaces: impl IntoIterator<Item = S>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            namespaces: namespaces.into_iter().map(|x| x.into()).collect(),
            ip: None,
            extras_queries: vec![],
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }

    pub fn ip(mut self, ip: IpValue) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn add_extras_query(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.extras_queries.push((name.into(), value.into()));
        self
    }

    pub fn extras_queries(
        mut self,
        extras_queries: Vec<(impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        self.extras_queries = extras_queries
            .into_iter()
            .map(|(n, v)| (n.into(), v.into()))
            .collect();
        self
    }

    pub(crate) fn create_notifications(&self) -> Vec<Notification> {
        self.namespaces
            .iter()
            .map(|namespace| Notification::new(namespace.clone()))
            .collect()
    }
}
