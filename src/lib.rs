#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![forbid(non_ascii_idents, unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*!
[![Rustc Version](https://img.shields.io/badge/rustc-1.39+-lightgray.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)
[![Actions](https://github.com/jmjoy/apollo-client/workflows/Rust/badge.svg?branch=master&event=push)](https://github.com/jmjoy/apollo-client/actions?query=workflow%3ARust+branch%3Amaster+event%3Apush++)
[![Crate](https://img.shields.io/crates/v/apollo-client.svg)](https://crates.io/crates/apollo-client)
[![API](https://docs.rs/apollo-client/badge.svg)](https://docs.rs/apollo-client)
[![Lines](https://img.shields.io/tokei/lines/github/jmjoy/apollo-client)](https://github.com/jmjoy/apollo-client)
[![License](https://img.shields.io/crates/l/apollo-client)](https://github.com/jmjoy/apollo-client/blob/master/LICENSE)

Rust🦀 client for [Ctrip Apollo](https://github.com/ctripcorp/apollo).

Power by Rust `async/await`.

# Installation

With [cargo edit](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add -s --features full tokio
$ cargo add -s --features full apollo-client
```

# Support

- [x] Fetch config via config service.
- [ ] Fetch config via mata service.
- [x] Apollo open apis.

# Features

1. Not all features are default, you can read the `[features]` section of [Cargo.toml](https://github.com/jmjoy/apollo-client/blob/master/Cargo.toml) to know all the features.

1. The `xml` and `yaml` features aren't enable by default, if you have such kind namespace, you should add `features` in `Cargo.toml`, just like:

    ```toml
    apollo-client = { version = "0.6", features = ["yaml", "xml"] }
    ```

    Or simply enable all features:

    ```toml
    apollo-client = { version = "0.6", features = ["full"] }
    ```

# Usage

You can find some examples in [the examples directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

# License

[Unlicense](https://github.com/jmjoy/apollo-client/blob/master/LICENSE).
*/

use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    io,
    ops::Deref,
    string::FromUtf8Error,
    time::Duration,
};

use futures::{
    future::{join_all, select, Either},
    pin_mut,
};
use indexmap::map::IndexMap;
#[cfg(feature = "regex")]
use regex::Regex;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

// #[cfg(feature = "open")]
// #[cfg_attr(docsrs, doc(cfg(feature = "open")))]
// pub use open::{OpenApiClient, OpenApiClientBuilder, OpenApiClientResult, OpenApiClientError};

#[macro_use]
pub mod common;
#[cfg(feature = "conf")]
#[cfg_attr(docsrs, doc(cfg(feature = "conf")))]
pub mod conf;
pub mod errors;
#[cfg(feature = "open")]
#[cfg_attr(docsrs, doc(cfg(feature = "open")))]
pub mod open;
#[cfg(test)]
mod tests;
mod utils;

/// Default request config url timeout.
const DEFAULT_CONFIG_TIMEOUT: Duration = Duration::from_secs(30);

/// Should be longer than server side's long polling timeout, which is now 60 seconds.
#[cfg(test)]
const DEFAULT_LISTEN_TIMEOUT: Duration = Duration::from_secs(3);
#[cfg(not(test))]
const DEFAULT_LISTEN_TIMEOUT: Duration = Duration::from_secs(90);

/// First listen timeout.
const FIRST_LISTEN_TIMEOUT: Duration = Duration::from_secs(3);

/// Apollo client crate side `Result`.
pub type ClientResult<T> = Result<T, ClientError>;

/// Apollo client crate side `Error`.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("Invalid uri: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("Serde json error: {0}")]
    SerdeJson(#[from] serde_json::error::Error),

    #[error("Serde urlencoded ser error: {0}")]
    SerdeUrlencodedSer(#[from] serde_urlencoded::ser::Error),

    #[cfg(feature = "yaml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "yaml")))]
    #[error("Serde yaml error: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[cfg(feature = "xml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
    #[error("Serde xml error: {0}")]
    SerdeXml(#[from] serde_xml_rs::Error),

    #[error("Empty responses")]
    EmptyResponses,

    #[error("Unknown apollo configuration kind: {0}")]
    UnknownApolloConfigurationKind(&'static str),

    #[error("Apollo content not found")]
    ApolloContentNotFound,

    #[error("Apollo config not found")]
    ApolloConfigNotFound,

    #[error("Apollo server error")]
    ApolloServerError,

    #[error("Apollo not modified")]
    ApolloNotModified,

    #[error("apollo other error, status code: {0}")]
    ApolloOtherError(u16),

    #[error("Apollo listen timeout")]
    ApolloListenTimeout,
}

/// Canonicalize the namespace. Just add `.properties` to the end of the namespace which not end
/// with `.properties` or `.xml` or `.json` or `.yaml` or `.yml` or `.txt`.
///
/// # Examples
///
/// ```rust
/// use apollo_client::canonicalize_namespace;
/// assert_eq!(canonicalize_namespace("foo"), "foo.properties");
/// assert_eq!(canonicalize_namespace("foo.yaml"), "foo.yaml");
/// ```
pub fn canonicalize_namespace(namespace: &str) -> String {
    if namespace.ends_with(".properties")
        || namespace.ends_with(".xml")
        || namespace.ends_with(".json")
        || namespace.ends_with(".yaml")
        || namespace.ends_with(".yml")
        || namespace.ends_with(".txt")
    {
        namespace.to_string()
    } else {
        format!("{}.properties", namespace)
    }
}

/// Configuration of Apollo and api information.
#[derive(Debug, Clone, PartialEq)]
pub struct ClientConfig<S: AsRef<str>, V: AsRef<[S]>> {
    pub config_server_url: S,
    pub app_id: S,
    pub cluster_name: S,
    pub namespace_names: V,
    pub ip: Option<IpValue<S>>,
}

impl Default for ClientConfig<&'static str, &'static [&'static str]> {
    fn default() -> Self {
        Self {
            config_server_url: "http://localhost:8080",
            app_id: "",
            cluster_name: "default",
            namespace_names: &["application"],
            ip: Default::default(),
        }
    }
}

impl Default for ClientConfig<&'static str, Vec<&'static str>> {
    fn default() -> Self {
        let client_config: ClientConfig<&'static str, &'static [&'static str]> = Default::default();
        Self {
            config_server_url: client_config.config_server_url,
            app_id: client_config.app_id,
            cluster_name: client_config.cluster_name,
            namespace_names: client_config.namespace_names.to_owned(),
            ip: Default::default(),
        }
    }
}

impl Default for ClientConfig<String, Vec<String>> {
    fn default() -> Self {
        let client_config: ClientConfig<&'static str, Vec<&'static str>> = Default::default();
        Self {
            config_server_url: client_config.config_server_url.to_owned(),
            app_id: client_config.app_id.to_owned(),
            cluster_name: client_config.cluster_name.to_owned(),
            namespace_names: client_config
                .namespace_names
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            ip: Default::default(),
        }
    }
}

/// Apollo config api `ip` param value.
#[derive(Debug, Clone, PartialEq)]
pub enum IpValue<S: AsRef<str>> {
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
    HostIpRegex(S),

    /// Specify your own IP address or other text.
    Custom(S),
}

impl<S: AsRef<str>> IpValue<S> {
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

/// Kind of a configuration namespace.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamespaceKind {
    Properties,
    Xml,
    Json,
    Yaml,
    Txt,
}

impl NamespaceKind {
    /// Infer the configuration namespace kind.
    pub fn infer_namespace_kind(namespace_name: &str) -> Self {
        if namespace_name.ends_with(".xml") {
            NamespaceKind::Xml
        } else if namespace_name.ends_with(".json") {
            NamespaceKind::Json
        } else if namespace_name.ends_with(".yml") || namespace_name.ends_with(".yaml") {
            NamespaceKind::Yaml
        } else if namespace_name.ends_with(".txt") {
            NamespaceKind::Txt
        } else {
            NamespaceKind::Properties
        }
    }
}

impl Display for NamespaceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Display::fmt(
            match self {
                NamespaceKind::Properties => "properties",
                NamespaceKind::Xml => "xml",
                NamespaceKind::Json => "json",
                NamespaceKind::Yaml => "yaml",
                NamespaceKind::Txt => "txt",
            },
            f,
        )
    }
}

/// Apollo config api responses.
#[derive(Debug)]
pub struct Responses {
    inner: Vec<ClientResult<Response>>,
}

impl Responses {
    fn from_bodies(bodies: Vec<ClientResult<String>>) -> Self {
        let inner = bodies
            .into_iter()
            .map(|body| body.and_then(|body| serde_json::from_str(&body).map_err(Into::into)))
            .collect();
        Self { inner }
    }

    pub fn into_inner(self) -> Vec<ClientResult<Response>> {
        self.inner
    }

    pub fn into_first(self) -> ClientResult<Response> {
        match self.into_inner().into_iter().nth(0) {
            Some(response) => response,
            None => Err(ClientError::EmptyResponses),
        }
    }

    pub fn into_vec_response(self) -> ClientResult<Vec<Response>> {
        self.into_inner().into_iter().collect()
    }

    pub fn into_map_response(self) -> ClientResult<HashMap<String, Response>> {
        Ok(self
            .into_vec_response()?
            .into_iter()
            .map(|response| (response.namespace_name.clone(), response))
            .collect())
    }
}

impl Deref for Responses {
    type Target = Vec<ClientResult<Response>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Apollo config api response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub app_id: String,
    pub cluster: String,
    pub namespace_name: String,
    pub configurations: IndexMap<String, String>,
    pub release_key: String,
}

impl Response {
    /// Get the `configurations.content` field of the response.
    pub fn get_configurations_content(&self) -> ClientResult<&str> {
        self.configurations
            .iter()
            .find_map(|(k, s)| {
                if k == "content" {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .ok_or(ClientError::ApolloContentNotFound)
    }

    /// Infer the configuration namespace kind.
    pub fn infer_kind(&self) -> NamespaceKind {
        let namespace_name = &self.namespace_name;

        if namespace_name.ends_with(".xml") {
            NamespaceKind::Xml
        } else if namespace_name.ends_with(".json") {
            NamespaceKind::Json
        } else if namespace_name.ends_with(".yml") || namespace_name.ends_with(".yaml") {
            NamespaceKind::Yaml
        } else if namespace_name.ends_with(".txt") {
            NamespaceKind::Txt
        } else {
            NamespaceKind::Properties
        }
    }

    /// Deserialize the `configurations` field for `properties`, or `configurations.content` for
    /// other namespace kind, without wrapper.
    pub fn deserialize_configurations<T: DeserializeOwned>(&self) -> ClientResult<T> {
        match self.infer_kind() {
            NamespaceKind::Properties => {
                let object = serde_json::Value::Object(
                    self.configurations
                        .iter()
                        .map(|(key, value)| (key.clone(), serde_json::Value::String(value.clone())))
                        .collect(),
                );
                Ok(serde_json::from_value(object)?)
            }
            NamespaceKind::Json => Ok(serde_json::from_str(self.get_configurations_content()?)?),
            #[cfg(feature = "yaml")]
            NamespaceKind::Yaml => Ok(serde_yaml::from_str(self.get_configurations_content()?)?),
            #[cfg(feature = "xml")]
            NamespaceKind::Xml => Ok(serde_xml_rs::from_str(self.get_configurations_content()?)?),
            NamespaceKind::Txt => {
                let value =
                    serde_json::Value::String(self.get_configurations_content()?.to_string());
                Ok(serde_json::from_value(value)?)
            }
            #[allow(unreachable_patterns)]
            k => panic!(
                "You have to enable feature `{}` for parsing this configuration kind.",
                k
            ),
        }
    }
}

type Notifications = Vec<Notification>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Notification {
    namespace_name: String,
    notification_id: i32,
}

fn initialize_notifications<S: AsRef<str>>(namespace_names: &[S]) -> Notifications {
    namespace_names
        .iter()
        .map(|namespace_name| Notification {
            namespace_name: {
                let mut namespace_name = namespace_name.as_ref();
                if namespace_name.ends_with(".properties") {
                    namespace_name = &namespace_name[..namespace_name.len() - ".properties".len()];
                }
                namespace_name.to_owned()
            },
            notification_id: -1,
        })
        .collect()
}

fn update_notifications(this: &mut Notifications, newer: Notifications) {
    for newer_item in newer {
        for this_item in this.iter_mut() {
            if this_item.namespace_name == newer_item.namespace_name {
                this_item.notification_id = newer_item.notification_id;
            }
        }
    }
}

/// Represents the apollo client.
pub struct Client<T: AsRef<str>, V: AsRef<[T]>> {
    client_config: ClientConfig<T, V>,
    notifications: Notifications,
    has_notify: bool,
}

impl<S: AsRef<str> + Display, V: AsRef<[S]>> Client<S, V> {
    /// New with the configuration of apollo and api parameters.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use apollo_client::{Client, ClientConfig};
    /// let client_config: ClientConfig<String, Vec<String>> = Default::default();
    /// let _ = Client::new(client_config);
    /// ```
    pub fn new(client_config: ClientConfig<S, V>) -> Self {
        let notifications = initialize_notifications(client_config.namespace_names.as_ref());
        Self {
            client_config,
            notifications,
            has_notify: false,
        }
    }

    /// Request apollo config api, and return response of your favorite type.
    pub async fn request(&self) -> ClientResult<Responses> {
        self.request_with_extras_query(None).await
    }

    /// Request apollo config api, and return response of your favorite type, with extras query.
    pub async fn request_with_extras_query(
        &self,
        extras_query: Option<&[(&str, &str)]>,
    ) -> ClientResult<Responses> {
        self.request_with_extras_query_and_namespaces(
            extras_query,
            &self.client_config.namespace_names,
        )
        .await
    }

    /// Request apollo config api, and return response of your favorite type, with extras query and
    /// specific namespaces.
    pub async fn request_with_extras_query_and_namespaces<Ns: AsRef<str>, Nv: AsRef<[Ns]>>(
        &self,
        extras_query: Option<&[(&str, &str)]>,
        namespace_names: Nv,
    ) -> ClientResult<Responses> {
        let namespace_names = namespace_names.as_ref();
        let mut futures = Vec::with_capacity(namespace_names.len());
        for namespace_name in namespace_names {
            let namespace_name = namespace_name.as_ref();
            futures.push(async move {
                let url = self.get_config_url(namespace_name, None, extras_query);
                match url {
                    Ok(url) => {
                        log::debug!("Request apollo config api: {}", &url);
                        Self::request_bodies(&url, DEFAULT_CONFIG_TIMEOUT).await
                    }
                    Err(e) => Err(e.into()),
                }
            });
        }
        let bodies = join_all(futures).await;
        log::trace!("Response apollo config data: {:?}", bodies);
        Ok(Responses::from_bodies(bodies))
    }

    async fn request_bodies(url: impl AsRef<str>, timeout: Duration) -> ClientResult<String> {
        imp::hyper::request_bodies(url.as_ref(), timeout).await
    }

    async fn sleep(dur: Duration) {
        imp::hyper::sleep(dur).await
    }

    /// Request apollo notification api just once.
    /// Return the namespace names if ok.
    pub async fn listen_once(&mut self) -> ClientResult<Vec<String>> {
        let url = self.get_listen_url(&self.notifications)?;
        log::debug!("Request apollo notifications api: {}", &url);

        let timeout = if self.has_notify {
            DEFAULT_LISTEN_TIMEOUT
        } else {
            FIRST_LISTEN_TIMEOUT
        };

        let fut1 = Self::request_bodies(url, timeout + Duration::from_secs(10));
        let fut2 = Self::sleep(timeout);
        pin_mut!(fut1);
        pin_mut!(fut2);

        let bodies = match select(fut1, fut2).await {
            Either::Left((bodies, ..)) => bodies?,
            Either::Right(_) => Err(ClientError::ApolloListenTimeout)?,
        };

        let notifications: Notifications = serde_json::from_str(&bodies)?;

        let notify_namespaces = notifications
            .iter()
            .map(|notification| notification.namespace_name.clone())
            .collect();

        update_notifications(&mut self.notifications, notifications);

        Ok(notify_namespaces)
    }

    /// Loop and request apollo notification api, if there is a change of the namespaces, return
    /// the response of your favorite type, or [`ClientError`] if there is something wrong.
    pub async fn listen_and_request(&mut self) -> ClientResult<Responses> {
        self.listen_and_request_with_extras_query(None).await
    }

    /// Loop and request apollo notification api, if there is a change of the namespaces, return
    /// the response of your favorite type, or [`ClientError`] if there is something wrong.
    pub async fn listen_and_request_with_extras_query(
        &mut self,
        extras_query: Option<&[(&str, &str)]>,
    ) -> ClientResult<Responses> {
        loop {
            match self.listen_once().await {
                Ok(namespaces) => {
                    return if self.has_notify {
                        self.request_with_extras_query_and_namespaces(extras_query, &namespaces)
                            .await
                    } else {
                        self.has_notify = true;
                        self.request_with_extras_query_and_namespaces(
                            extras_query,
                            self.client_config.namespace_names.as_ref(),
                        )
                        .await
                    };
                }
                Err(ClientError::ApolloNotModified) => {}
                Err(ClientError::ApolloListenTimeout) => {
                    if !self.has_notify {
                        self.has_notify = true;

                        return self
                            .request_with_extras_query_and_namespaces(
                                extras_query,
                                self.client_config.namespace_names.as_ref(),
                            )
                            .await;
                    }
                }
                Err(e) => Err(e)?,
            }
        }
    }

    fn get_config_url(
        &self,
        namespace_name: &str,
        release_key: Option<&str>,
        extras_query: Option<&[(&str, &str)]>,
    ) -> Result<String, serde_urlencoded::ser::Error> {
        let mut query = Vec::new();
        if let Some(release_key) = release_key {
            query.push(("releaseKey", release_key));
        }
        if let Some(ip) = &self.client_config.ip {
            query.push(("ip", ip.to_str()));
        }
        if let Some(extras_query) = extras_query {
            for item in extras_query {
                query.push(item.to_owned());
            }
        }

        let mut query = serde_urlencoded::to_string(query)?;
        if !query.is_empty() {
            query.insert(0, '?');
        }

        Ok(format!(
            "{config_server_url}/configs/{app_id}/{cluster_name}/{namespace_name}{query}",
            config_server_url = self.client_config.config_server_url,
            app_id = self.client_config.app_id,
            cluster_name = self.client_config.cluster_name,
            namespace_name = namespace_name,
            query = query,
        ))
    }

    fn get_listen_url(&self, notifications: &Notifications) -> ClientResult<String> {
        let notifications = if notifications.len() > 0 {
            let notifications = &[("notifications", serde_json::to_string(&notifications)?)];
            let mut notifications = serde_urlencoded::to_string(notifications)?;
            notifications.insert(0, '&');
            notifications
        } else {
            "".to_string()
        };

        Ok(format!(
            "{config_server_url}/notifications/v2?appId={app_id}&cluster={cluster_name}{notifications}",
            config_server_url = self.client_config.config_server_url,
            app_id = self.client_config.app_id,
            cluster_name = self.client_config.cluster_name,
            notifications = notifications,
        ))
    }
}

pub(crate) mod imp {
    pub(crate) mod hyper {
        use std::{str, time::Duration};

        use futures::{
            future::{select, Either},
            pin_mut,
        };
        use hyper::{body, body::Buf, client::Client, StatusCode};

        use crate::{ClientError, ClientResult};

        pub(crate) async fn sleep(dur: Duration) {
            tokio::time::sleep(dur).await
        }

        pub(crate) async fn request_bodies(url: &str, timeout: Duration) -> ClientResult<String> {
            let client = Client::new();

            let fut1 = client.get(url.parse()?);
            let fut2 = sleep(timeout);
            pin_mut!(fut1);
            pin_mut!(fut2);

            let response = match select(fut1, fut2).await {
                Either::Left((response, ..)) => response?,
                Either::Right(_) => return Err(ClientError::ApolloServerError),
            };
            handle_response_status(&response)?;
            let mut buf = body::aggregate(response).await?;

            let mut bodies = Vec::new();
            while buf.has_remaining() {
                let chunk = buf.chunk();
                let len = chunk.len();
                bodies.extend_from_slice(chunk);
                buf.advance(len);
            }

            Ok(String::from_utf8(bodies)?)
        }

        fn handle_response_status<T>(response: &hyper::Response<T>) -> ClientResult<()> {
            let status = response.status();
            if !status.is_success() {
                match response.status() {
                    StatusCode::NOT_MODIFIED => Err(ClientError::ApolloNotModified)?,
                    StatusCode::NOT_FOUND => Err(ClientError::ApolloConfigNotFound)?,
                    StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::ApolloServerError)?,
                    status => Err(ClientError::ApolloOtherError(status.as_u16()))?,
                }
            }
            Ok(())
        }
    }
}
