//! Common api metadata.

use crate::errors::{ApolloClientResult, ApolloResponseError};
use async_trait::async_trait;
use http::Method;
use reqwest::{RequestBuilder, Response};
use std::{borrow::Cow, fmt, fmt::Display, time::Duration};
use url::Url;

#[allow(dead_code)]
pub(crate) const DEFAULT_CLUSTER_NAME: &str = "default";
#[allow(dead_code)]
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
#[allow(dead_code)]
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

/// Common api request trait.
pub(crate) trait PerformRequest {
    /// The returned response after request is success.
    type Response: PerformResponse;

    /// Url path.
    fn path(&self) -> String;

    /// Request method.
    fn method(&self) -> http::Method {
        Method::GET
    }

    /// Url queries.
    fn queries(&self) -> ApolloClientResult<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        Ok(vec![])
    }

    /// Handle extras operator, such as set request body.
    #[allow(unused_mut)]
    fn request_builder(&self, mut request_builder: RequestBuilder) -> RequestBuilder {
        //FIXME
        //see issue #15701 <https://github.com/rust-lang/rust/issues/15701>
        #[cfg(all(feature = "auth", feature = "conf"))]
        {
            request_builder = self.signature(request_builder);
        }
        request_builder
    }

    /// AppId
    fn app_id(&self) -> Option<&str> {
        None
    }

    /// Access key
    #[cfg(all(feature = "auth", feature = "conf"))]
    fn access_key(&self) -> Option<&str> {
        None
    }

    /// make `Authorization` header
    ///
    /// # Documentation
    ///
    /// https://www.apolloconfig.com/#/zh/usage/other-language-client-user-guide?id=_15-%e9%85%8d%e7%bd%ae%e8%ae%bf%e9%97%ae%e5%af%86%e9%92%a5
    #[cfg(all(feature = "auth", feature = "conf"))]
    fn signature(&self, mut request_builder: RequestBuilder) -> RequestBuilder {
        use hmac::{Mac, SimpleHmac};
        use sha1::Sha1;
        type HmacWithSha1 = SimpleHmac<Sha1>;
        if let (Some(app_id), Some(access_key)) = (self.app_id(), self.access_key()) {
            let ts = chrono::Utc::now().timestamp_millis();
            let mut url = self.path();
            if let Ok(queries) = self.queries() {
                if !queries.is_empty() {
                    url += "?";
                    for (idx, (key, val)) in queries.into_iter().enumerate() {
                        url += &format!(
                            "{}{}={}",
                            if idx > 0 { "&" } else { "" },
                            urlencoding::encode(&key),
                            urlencoding::encode(&val)
                        );
                    }
                }
            }
            if let Ok(mut hmac) = HmacWithSha1::new_from_slice(access_key.as_bytes()) {
                hmac.update(format!("{}\n{}", ts, url).as_bytes());
                let sign = base64::encode(hmac.finalize().into_bytes());
                request_builder = request_builder
                    .header(
                        reqwest::header::AUTHORIZATION,
                        format!("Apollo {}:{}", app_id, sign),
                    )
                    .header("Timestamp", ts);
            }
        }
        request_builder
    }
}

/// Common api response trait.
#[async_trait]
pub(crate) trait PerformResponse: Sized {
    /// Create Self from response.
    async fn from_response(response: Response) -> ApolloClientResult<Self>;
}

#[async_trait]
impl PerformResponse for () {
    async fn from_response(_response: Response) -> ApolloClientResult<Self> {
        Ok(())
    }
}

#[cfg(feature = "conf")]
#[async_trait]
impl PerformResponse for ini::Properties {
    async fn from_response(response: Response) -> ApolloClientResult<Self> {
        let content = response.text().await?;
        let i = ini::Ini::load_from_str(&content)?;
        Ok(i.section(None::<&'static str>)
            .ok_or(crate::errors::ApolloClientError::EmptyConfiguration)?
            .clone())
    }
}

/// Create request url from base url, mainly path and queries.
#[allow(dead_code)]
pub(crate) fn handle_url(request: &impl PerformRequest, base_url: Url) -> ApolloClientResult<Url> {
    let mut url = base_url;
    let path = &request.path();
    let query = &request.queries()?;

    url.path_segments_mut()
        .map_err(|_| crate::errors::ApolloClientError::UrlCannotBeABase)?
        .extend(path.split('/'));
    if !query.is_empty() {
        url.query_pairs_mut().extend_pairs(query);
    }

    Ok(url)
}

/// Validate response is successful or not.
#[allow(dead_code)]
pub(crate) async fn validate_response(response: Response) -> ApolloClientResult<Response> {
    ApolloResponseError::from_response(response)
        .await
        .map_err(Into::into)
}

/// Implement PerformResponse for response struct which content type is `application/json`.
#[allow(unused_macros)]
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
