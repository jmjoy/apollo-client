use crate::errors::{
    ApolloClientError, ApolloClientError::UrlCannotBeABase, ApolloClientResult, ApolloResponseError,
};
use async_trait::async_trait;
use http::Method;
use ini::{Ini, Properties};
use reqwest::{RequestBuilder, Response};
use std::{borrow::Cow, fmt, fmt::Display, time::Duration};
use url::Url;

pub(crate) const DEFAULT_CLUSTER_NAME: &str = "default";
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
pub(crate) const DEFAULT_NOTIFY_TIMEOUT: Duration = Duration::from_secs(90);

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

pub trait PerformRequest {
    type Response: PerformResponse;

    fn path(&self) -> String;

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        Ok(vec![])
    }

    fn request_builder(&self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder
    }
}

#[async_trait]
pub trait PerformResponse: Sized {
    async fn from_response(response: Response) -> ApolloClientResult<Self>;
}

#[async_trait]
impl PerformResponse for Properties {
    async fn from_response(response: Response) -> ApolloClientResult<Self> {
        let content = response.text().await?;
        let i = Ini::load_from_str(&content)?;
        Ok(i.section(None::<&'static str>)
            .ok_or(ApolloClientError::EmptyConfiguration)?
            .clone())
    }
}

pub(crate) fn handle_url(request: &impl PerformRequest, base_url: Url) -> ApolloClientResult<Url> {
    let mut url = base_url;
    let path = &request.path();
    let query = &request.queries()?;

    url.path_segments_mut()
        .map_err(|_| UrlCannotBeABase)?
        .extend(path.split('/'));
    if !query.is_empty() {
        url.query_pairs_mut().extend_pairs(query);
    }

    Ok(url)
}

pub(crate) fn validate_response(response: &Response) -> ApolloClientResult<()> {
    match ApolloResponseError::from_status_code(response.status()) {
        Some(e) => Err(e.into()),
        None => Ok(()),
    }
}

macro_rules! implement_json_perform_response_for {
    ($t:ty) => {
        #[async_trait::async_trait]
        impl $crate::meta::PerformResponse for $t {
            async fn from_response(
                response: ::reqwest::Response,
            ) -> $crate::errors::ApolloClientResult<Self> {
                Ok(response.json().await?)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_namespace_kind() {
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.properties"),
            NamespaceKind::Properties
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.xml"),
            NamespaceKind::Xml
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.yaml"),
            NamespaceKind::Yaml
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.yml"),
            NamespaceKind::Yaml
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.json"),
            NamespaceKind::Json
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo.txt"),
            NamespaceKind::Txt
        );
        assert_eq!(
            NamespaceKind::infer_namespace_kind("foo"),
            NamespaceKind::Properties
        );
    }
}
