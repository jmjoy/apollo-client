use crate::requests::PerformRequest;
use crate::utils;
use regex::Regex;
use std::borrow::Cow;
use serde::de::DeserializeOwned;

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
pub struct CachedConfRequest {
    app_id: String,
    cluster_name: Cow<'static, str>,
    namespace_name: String,
    ip: Option<IpValue>,
    extras_query: Vec<(Cow<'static, str>, Cow<'static, str>)>
}

impl CachedConfRequest {
    pub fn new(
        app_id: String,
        cluster_name: impl Into<Cow<'static, str>>,
        namespace_name: String,
        ip: Option<IpValue>,
        extras_query: Vec<(impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        Self {
            app_id,
            cluster_name: cluster_name.into(),
            namespace_name,
            ip,
            extras_query: extras_query.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
        }
    }
}

impl PerformRequest for CachedConfRequest {
    type Response = ();

    fn path(&self) -> String {
        format!("/configfiles/{app_id}/{cluster_name}/{namespace_name}",
                app_id = self.app_id, cluster_name = self.cluster_name, namespace_name = self.namespace_name)
    }

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        // let mut pairs = vec![];
        // pairs.insert()
        todo!()
    }
}
