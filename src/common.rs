use crate::errors::{
    ApolloClientError, ApolloClientError::UrlCannotBeABase, ApolloClientResult, ApolloResponseError,
};
use async_trait::async_trait;
use http::Method;
use ini::{Ini, Properties};
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::{borrow::Cow, collections::HashMap};
use url::Url;

pub(crate) const DEFAULT_CLUSTER_NAME: &'static str = "default";

pub trait PerformRequest {
    type Response: PerformResponse;

    fn path(&self) -> String;

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        vec![]
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
            .ok_or(ApolloClientError::EmptyConfig)?
            .clone())
    }
}

pub(crate) fn handle_url(request: &impl PerformRequest, base_url: Url) -> ApolloClientResult<Url> {
    let mut url = base_url;
    let path = &request.path();
    let query = &request.query();

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

macro_rules! implement_json_perform_response {
    ($t:ty) => {
        #[async_trait::async_trait]
        impl $crate::common::PerformResponse for $t {
            async fn from_response(
                response: ::reqwest::Response,
            ) -> $crate::errors::ApolloClientResult<Self> {
                Ok(response.json().await?)
            }
        }
    };
}
