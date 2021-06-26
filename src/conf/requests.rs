use crate::{
    common::{PerformRequest, DEFAULT_CLUSTER_NAME},
    utils,
};
use ini::Properties;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::{borrow::Cow, collections::HashMap};

/// Apollo config api `ip` param value.
#[derive(Debug, Clone, PartialEq)]
pub enum IpValue {
    /// Get the hostname of the machine.
    #[cfg(feature = "host-name")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-name")))]
    HostName,

    /// Get the first ip of the machine generally.
    #[cfg(feature = "host-ip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-ip")))]
    HostIp,

    /// Get the first ip of the machine match the prefix, such as `^10\.2\.`.
    #[cfg(feature = "host-ip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-ip")))]
    HostIpRegex(String),

    /// Specify your own IP address or other text.
    Custom(String),
}

impl IpValue {
    fn to_str(&self) -> &str {
        match self {
            #[cfg(feature = "host-name")]
            IpValue::HostName => utils::get_hostname(),

            #[cfg(feature = "host-ip")]
            IpValue::HostIp => utils::get_all_addrs()
                .iter()
                .find(|addr| !addr.starts_with("127.") && addr.as_str() != "::1")
                .map(|s| s.as_str())
                .unwrap_or("127.0.0.1"),

            #[cfg(feature = "host-ip")]
            IpValue::HostIpRegex(regex) => {
                let re = Regex::new(regex.as_ref()).expect("Parse regex of HostIpRegex failed");
                utils::get_all_addrs()
                    .iter()
                    .find(|addr| re.is_match(addr))
                    .map(|s| s.as_str())
                    .unwrap_or("127.0.0.1")
            }

            IpValue::Custom(s) => s.as_ref(),
        }
    }
}

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

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut pairs = vec![];
        if let Some(ip) = &self.ip {
            pairs.push(("ip".into(), ip.to_str().to_owned().into()));
        }
        if !self.extras_queries.is_empty() {
            pairs.extend_from_slice(&self.extras_queries);
        }
        pairs
    }
}

impl PerformConfRequest for CachedFetchRequest {}

#[derive(Clone, Debug)]
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
}

impl PerformRequest for FetchRequest {
    type Response = Properties;

    fn path(&self) -> String {
        format!(
            "/configs/{app_id}/{cluster_name}/{namespace_name}",
            app_id = self.app_id,
            cluster_name = self.cluster_name,
            namespace_name = self.namespace_name
        )
    }

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        let mut pairs = vec![];
        if let Some(ip) = &self.ip {
            pairs.push(("ip".into(), ip.to_str().to_owned().into()));
        }
        if let Some(release_key) = &self.release_key {
            pairs.push(("releaseKey".into(), release_key.clone().into()));
        }
        if !self.extras_queries.is_empty() {
            pairs.extend_from_slice(&self.extras_queries);
        }
        pairs
    }
}

impl PerformConfRequest for FetchRequest {}
