/*!
Rust🦀 client for [Apollo](https://github.com/ctripcorp/apollo).

Power by Rust `async/await`.

## Installation

With [cargo add](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add -s apollo-client
```

## Features

1. Not all features are default, you can read the `[features]` section of [Cargo.toml](https://github.com/jmjoy/apollo-client/blob/master/Cargo.toml) to know all the features.

1. The `xml` and `yaml` features aren't enable by default, if you have such kind namespace, you should add `features` in `Cargo.toml`, just like:

    ```toml
    apollo-client = { version = "0.5", features = ["yaml", "xml"] }
    ```

    Or simply enable all features:

    ```toml
    apollo-client = { version = "0.5", features = ["full"] }
    ```

1. By default, using curl client `isahc` to handle http request, you can switch to `hyper` by enable the `with-hyper` feature.

    ```toml
    apollo-client = { version = "0.5", default-features = false, features = ["with-hyper"] }
    ```

    Or:

    ```toml
    apollo-client = { version = "0.5", default-features = false, features = ["full-hyper"] }
    ```

    Or specify the `Scenario`.


## Usage

You can find some examples in [the examples directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

## License

Unlicense.
*/
use futures::{
    future::{join_all, select, Either},
    pin_mut,
};
use indexmap::map::IndexMap;
use quick_error::quick_error;
use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    io,
    ops::Deref,
    string::FromUtf8Error,
    time::Duration,
};

// use isahc::config::{DnsCache, VersionNegotiation};
#[cfg(feature = "regex")]
use regex::Regex;

#[cfg(test)]
mod tests;

/// Default request config url timeout.
const DEFAULT_CONFIG_TIMEOUT: Duration = Duration::from_secs(30);

/// Should be longer than server side's long polling timeout, which is now 60 seconds.
#[cfg(feature = "mock-listen")]
const DEFAULT_LISTEN_TIMEOUT: Duration = Duration::from_secs(3);
#[cfg(not(feature = "mock-listen"))]
const DEFAULT_LISTEN_TIMEOUT: Duration = Duration::from_secs(90);

/// First listen timeout.
const FIRST_LISTEN_TIMEOUT: Duration = Duration::from_secs(3);

/// Apollo client crate side `Result`.
pub type ClientResult<T> = Result<T, ClientError>;

quick_error! {
    /// Apollo client crate side `Error`.
    #[derive(Debug)]
    pub enum ClientError {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Utf8(err: FromUtf8Error) {
            from()
            description("utf-8 error")
            display("UTF-8 error: {}", err)
            cause(err)
        }
        Hyper(err: hyper::Error) {
            description("hyper error")
            display("Hyper error: {}", err)
            cause(err)
        }
        InvalidUri(err: http::uri::InvalidUri) {
            description("invalid uri")
            display("Invalid uri: {}", err)
            cause(err)
        }
        SerdeJson(err: serde_json::error::Error) {
            from()
            description("serde json error")
            display("Serde json error: {}", err)
            cause(err)
        }
        SerdeUrlencodedSer(err: serde_urlencoded::ser::Error) {
            from()
            description("serde urlencoded ser error")
            display("Serde urlencoded ser error: {}", err)
            cause(err)
        }
        #[cfg(feature = "yaml")]
        SerdeYaml(err: serde_yaml::Error) {
            description("serde yaml error")
            display("Serde yaml error: {}", err)
            cause(err)
        }
        #[cfg(feature = "xml")]
        SerdeXml(err: serde_xml_rs::Error) {
            description("serde xml error")
            display("Serde xml error: {}", err)
            cause(err)
        }
        EmptyResponses {
            description("empty responses")
            display("Empty responses")
        }
        UnknownApolloConfigurationKind(kind: &'static str) {
            description("unknown apollo configuration kind")
            display("Unknown apollo configuration kind: {}", kind)
        }
        ApolloContentNotFound {
            description("apollo content not found")
            display("Apollo content not found")
        }
        ApolloConfigNotFound {
            description("apollo config not found")
            display("Apollo config not found")
        }
        ApolloServerError {
            description("apollo server error")
            display("Apollo server error")
        }
        ApolloNotModified {
            description("apollo not modified")
            display("Apollo not modified")
        }
        ApolloOtherError(code: u16) {
            description("apollo other error")
            display("apollo other error, status code: {}", code)
        }
        ApolloListenTimeout {
            description("apollo listen timeout")
            display("Apollo listen timeout")
        }
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for ClientError {
    fn from(err: serde_yaml::Error) -> ClientError {
        ClientError::SerdeYaml(err)
    }
}

#[cfg(feature = "xml")]
impl From<serde_xml_rs::Error> for ClientError {
    fn from(err: serde_xml_rs::Error) -> ClientError {
        ClientError::SerdeXml(err)
    }
}

impl From<hyper::Error> for ClientError {
    fn from(err: hyper::Error) -> ClientError {
        ClientError::Hyper(err)
    }
}

impl From<http::uri::InvalidUri> for ClientError {
    fn from(err: http::uri::InvalidUri) -> ClientError {
        ClientError::InvalidUri(err)
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientConfig<S: AsRef<str>, V: AsRef<[S]>> {
    pub config_server_url: S,
    pub app_id: S,
    pub cluster_name: S,
    pub namespace_names: V,
    #[serde(default)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IpValue<S: AsRef<str>> {
    /// Get the hostname of the machine.
    #[cfg(feature = "host-name")]
    HostName,

    /// Get the first ip of the machine generally.
    HostIp,

    /// Get the first ip of the machine match the prefix, such as `^10\.2\.`.
    #[cfg(feature = "host-ip")]
    HostIpRegex(S),

    /// Specify your own IP address or other text.
    Custom(S),
}

impl<S: AsRef<str>> IpValue<S> {
    fn to_str(&self) -> &str {
        match self {
            #[cfg(feature = "host-name")]
            IpValue::HostName => get_hostname(),

            #[cfg(feature = "host-ip")]
            IpValue::HostIp => get_all_addrs()
                .iter()
                .find(|addr| !addr.starts_with("127.") && addr.as_str() != "::1")
                .map(|s| s.as_str())
                .unwrap_or("127.0.0.1"),

            #[cfg(feature = "host-ip")]
            IpValue::HostIpRegex(regex) => {
                let re = Regex::new(regex.as_ref()).expect("Parse regex of HostIpRegex failed");
                get_all_addrs()
                    .iter()
                    .find(|addr| re.is_match(addr))
                    .map(|s| s.as_str())
                    .unwrap_or("127.0.0.1")
            }

            IpValue::Custom(s) => s.as_ref(),
        }
    }
}

#[cfg(feature = "host-name")]
fn get_hostname() -> &'static str {
    use once_cell::sync::OnceCell;
    static HOST_NAME: OnceCell<String> = OnceCell::new();
    HOST_NAME.get_or_init(|| {
        hostname::get()
            .ok()
            .and_then(|hostname| hostname.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
    })
}

#[cfg(feature = "host-ip")]
fn get_all_addrs() -> &'static [String] {
    use once_cell::sync::OnceCell;
    use systemstat::{data::IpAddr, platform::common::Platform, System};

    static ALL_ADDRS: OnceCell<Vec<String>> = OnceCell::new();
    ALL_ADDRS.get_or_init(|| {
        System::new()
            .networks()
            .ok()
            .map(|networks| {
                networks
                    .values()
                    .map(|network| {
                        network
                            .addrs
                            .iter()
                            .filter_map(|network_addr| match network_addr.addr {
                                IpAddr::V4(addr) => Some(addr.to_string()),
                                IpAddr::V6(addr) => Some(addr.to_string()),
                                _ => None,
                            })
                    })
                    .flatten()
                    .collect()
            })
            .unwrap_or(Vec::new())
    })
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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
        use crate::{ClientError, ClientResult};
        use futures::{
            future::{select, Either},
            pin_mut,
        };
        use hyper::{body, body::Buf, client::Client, StatusCode};
        use std::{str, time::Duration};

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
