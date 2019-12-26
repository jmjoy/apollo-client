use futures::future::try_join_all;
use isahc::get_async;
use isahc::ResponseExt;
use lazy_static::lazy_static;
use quick_error::quick_error;
use serde::de::{DeserializeOwned, MapAccess, SeqAccess};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::slice::SliceIndex;
use std::str::FromStr;
use std::task::{Context, Poll};
use std::{fmt, io};

#[cfg(test)]
mod tests;

pub type ApolloClientResult<T> = Result<T, ApolloClientError>;

quick_error! {
    #[derive(Debug)]
    pub enum ApolloClientError {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        Isahc(err: isahc::Error) {
            from()
            description("isahc error")
            display("Isahc error: {}", err)
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
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for ApolloClientError {
    fn from(err: serde_yaml::Error) -> ApolloClientError {
        ApolloClientError::SerdeYaml(err)
    }
}

#[cfg(feature = "xml")]
impl From<serde_xml_rs::Error> for ApolloClientError {
    fn from(err: serde_xml_rs::Error) -> ApolloClientError {
        ApolloClientError::SerdeXml(err)
    }
}

const NOTIFY_URL_TPL: &'static str = "{config_server_url}/notifications/v2?appId={appId}&cluster={clusterName}&notifications={notifications}";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig<'a> {
    pub config_server_url: &'a str,
    pub app_id: &'a str,
    pub cluster_name: &'a str,
    pub namespace_names: Vec<&'a str>,
    #[serde(default)]
    pub ip: Option<IpValue<'a>>,
}

impl Default for ClientConfig<'_> {
    fn default() -> Self {
        Self {
            config_server_url: "http://localhost:8080",
            app_id: "",
            cluster_name: "default",
            namespace_names: vec!["application"],
            ip: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum IpValue<'a> {
    HostName,
    Custom(&'a str),
}

impl<'a> IpValue<'a> {
    fn to_str(&'a self) -> &'a str {
        match self {
            IpValue::HostName => {
                lazy_static! {
                    static ref HOSTNAME: String = {
                        hostname::get()
                            .map_err(|_| ())
                            .and_then(|hostname| hostname.into_string().map_err(|_| ()))
                            .unwrap_or_else(|_| "unknown".to_string())
                    };
                }
                &HOSTNAME
            }
            IpValue::Custom(s) => s,
        }
    }
}

pub trait FromResponses: Sized {
    type Err;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err>;
}

impl FromResponses for Response {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        Ok(responses
            .into_iter()
            .nth(0)
            .ok_or(ApolloClientError::EmptyResponses)?)
    }
}

impl FromResponses for Vec<Response> {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        Ok(responses)
    }
}

impl FromResponses for HashMap<String, Response> {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        let mut m = HashMap::with_capacity(responses.len());
        for response in responses {
            m.insert(response.namespace_name.clone(), response);
        }
        Ok(m)
    }
}

impl<T: DeserializeOwned> FromResponses for Configuration<T> {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        Response::from_responses(responses)?.deserialize_to_configuration()
    }
}

impl<T: DeserializeOwned> FromResponses for Vec<Configuration<T>> {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        responses
            .into_iter()
            .map(|response| response.deserialize_to_configuration())
            .collect()
    }
}

impl<T: DeserializeOwned> FromResponses for HashMap<String, Configuration<T>> {
    type Err = ApolloClientError;

    fn from_responses(responses: Vec<Response>) -> Result<Self, Self::Err> {
        <HashMap<String, Response>>::from_responses(responses)?
            .into_iter()
            .map(|(key, response)| {
                response
                    .deserialize_to_configuration()
                    .map(|configuration| (key, configuration))
            })
            .collect()
    }
}

pub struct Configuration<T> {
    inner: T,
}

impl<T> Configuration<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Configuration<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Configuration<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Debug> Debug for Configuration<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(&format!("Configuration {{ {:?} }}", &self.inner), f)
    }
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub cluster: String,
    #[serde(rename = "namespaceName")]
    pub namespace_name: String,
    pub configurations: HashMap<String, String>,
    #[serde(rename = "releaseKey")]
    pub release_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigurationKind {
    Properties,
    Xml,
    Json,
    Yaml,
    Txt,
}

impl Display for ConfigurationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Display::fmt(
            match self {
                ConfigurationKind::Properties => "properties",
                ConfigurationKind::Xml => "xml",
                ConfigurationKind::Json => "json",
                ConfigurationKind::Yaml => "yaml",
                ConfigurationKind::Txt => "txt",
            }, f )
    }
}

impl Response {
    pub fn get_configurations_content(&self) -> ApolloClientResult<&str> {
        self.configurations
            .get("content")
            .map(|s| s.as_str())
            .ok_or(ApolloClientError::ApolloContentNotFound)
    }

    pub fn infer_kind(&self) -> ConfigurationKind {
        let namespace_name = &self.namespace_name;

        if namespace_name.ends_with(".xml") {
            ConfigurationKind::Xml
        } else if namespace_name.ends_with(".json") {
            ConfigurationKind::Json
        } else if namespace_name.ends_with(".yml") || namespace_name.ends_with(".yaml") {
            ConfigurationKind::Yaml
        } else if namespace_name.ends_with(".txt") {
            ConfigurationKind::Txt
        } else {
            ConfigurationKind::Properties
        }
    }

    pub fn deserialize_configuration<'de, T: DeserializeOwned>(&self,) -> ApolloClientResult<T> {
        match self.infer_kind() {
            ConfigurationKind::Json => {
                Ok(serde_json::from_str(self.get_configurations_content()?)?)
            }
            #[cfg(feature = "yaml")]
            ConfigurationKind::Yaml => {
                Ok(serde_yaml::from_str(self.get_configurations_content()?)?)
            }
            #[cfg(feature = "xml")]
            ConfigurationKind::Xml => {
                Ok(serde_xml_rs::from_str(self.get_configurations_content()?)?)
            }
            k => panic!(
                "You have to enable feature `{}` for parsing this configuration kind.",
                k
            ),
        }
    }

    pub fn deserialize_to_configuration<T: DeserializeOwned>(&self) -> ApolloClientResult<Configuration<T>> {
        self.deserialize_configuration()
            .map(|inner| Configuration::new(inner))
    }
}

pub struct Client<'a> {
    client_config: &'a ClientConfig<'a>,
}

impl<'a> Client<'a> {
    pub fn new_with_config(client_config: &'a ClientConfig<'a>) -> Self {
        Self { client_config }
    }

    pub async fn request<T: FromResponses<Err = ApolloClientError>>(
        &self,
    ) -> ApolloClientResult<T> {
        let mut futures = Vec::with_capacity(self.client_config.namespace_names.len());
        for (index, namespace_name) in self.client_config.namespace_names.iter().enumerate() {
            let url = self.get_config_url(namespace_name, None)?;
            futures.push(async move { Self::request_response(&url).await });
        }
        let responses = try_join_all(futures).await?;
        FromResponses::from_responses(responses)
    }

    async fn request_response(url: &str) -> ApolloClientResult<Response> {
        let body = get_async(url).await?.text_async().await?;
        log::trace!("Receive response body: {}", body);
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn listen() {
        todo!()
    }

    fn get_config_url(
        &self,
        namespace_name: &str,
        release_key: Option<&str>,
    ) -> Result<String, serde_urlencoded::ser::Error> {
        let mut query = Vec::new();
        if let Some(release_key) = release_key {
            query.push(("release_key", release_key));
        }
        if let Some(ip) = &self.client_config.ip {
            query.push(("ip", ip.to_str()));
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
}
