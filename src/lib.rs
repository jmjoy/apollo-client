use serde::de::{MapAccess, SeqAccess};
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::future::Future;
use std::task::{Context, Poll};
use std::str::FromStr;
use std::slice::SliceIndex;
use quick_error::quick_error;

pub type ApolloClientResult<T> = Result<T, ApolloClientError>;

quick_error! {
    #[derive(Debug)]
    pub enum ApolloClientError {
        EmptyResponses {
            description("empty responses")
            display("Empty responses")
        }
    }
}

const NOTIFY_URL_TPL: &'static str = "{config_server_url}/notifications/v2?appId={appId}&cluster={clusterName}&notifications={notifications}";

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub enum IpValue<'a> {
    HostIp,
    HostName,
    Custom(&'a str),
}

impl<'a> ToString for IpValue<'a> {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

//impl<'a> FromStr for IpValue<'a> {
//    type Err = ();
//
//    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
//        match s {
//            "<HOST_IP>" => Ok(IpValue::HostIp),
//            "<HOST_NAME>" => Ok(IpValue::HostName),
//            s => Ok(IpValue::Custom(s)),
//        }
//    }
//}

pub trait FromResponses<'a>: Sized {
    type Err;

    fn from_responses(responses: &'a [Response<'a>]) -> Result<Self, Self::Err>;
}

impl<'a> FromResponses<'a> for &'a Response<'a> {
    type Err = ApolloClientError;

    fn from_responses(responses: &'a [Response]) -> Result<Self, Self::Err> {
        Ok(responses.iter().nth(0).ok_or(ApolloClientError::EmptyResponses)?)
    }
}

impl<'a> FromResponses<'a> for HashMap<&'a str, &'a Response<'a>> {
    type Err = ApolloClientError;

    fn from_responses(responses: &'a [Response<'a>]) -> Result<Self, Self::Err> {
        let mut m = HashMap::with_capacity(responses.len());
        for response in responses {
            m.insert(response.namespace_name, response);
        }
        Ok(m)
    }
}

#[derive(Debug, Serialize)]
pub struct Response<'a> {
    #[serde(rename = "appId")]
    app_id: &'a str,
    cluster: &'a str,
    #[serde(rename = "namespaceName")]
    namespace_name: &'a str,
    configurations: ResponseConfigurations<'a>,
    #[serde(rename = "releaseKey")]
    release_key: &'a str,
}

#[derive(Debug, Serialize)]
pub enum ResponseConfigurations<'a> {
    RawString(&'a str),
    PropertiesMap(HashMap<&'a str, &'a str>),
}

pub struct Client<'a> {
    client_config: &'a ClientConfig<'a>,
}

impl<'a> Client<'a> {
    pub fn new_with_config(client_config: &'a ClientConfig<'a>) -> Self {
        Self {
            client_config,
        }
    }

    pub async fn listen() {
        todo!()
    }

    pub async fn request<T: FromResponses<'a>>(&self) -> T {
        self.client_config.namespace_names.iter().map(|namespace_name| self.get_config_url(*namespace_name, None));
//            .map(|config_url| isahc);

        todo!()
    }

    pub async fn get_config() -> &'a Response<'a> {
        todo!()
    }

    fn get_config_url(&self, namespace_name: &str, release_key: Option<&str>) -> String {
        let mut query = String::new();
        let params = [
            if let Some(release_key) = release_key {
                format!("release_key={}", release_key)
            } else {
                "".to_string()
            },
            if let Some(ip) = &self.client_config.ip {
                format!("release_key={}", ip.to_string())
            } else {
                "".to_string()
            }
        ];
        let params = params.join("&");
        if !params.is_empty() {
            query.push('?');
            query.push_str(&params);
        }

        format!(
            "{config_server_url}/configs/{app_id}/{cluster_name}/{namespace_name}{query}",
            config_server_url = self.client_config.config_server_url,
            app_id = self.client_config.app_id,
            cluster_name = self.client_config.cluster_name,
            namespace_name = namespace_name,
            query = query,
        )
    }
}

