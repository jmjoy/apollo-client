use crate::{common::PerformResponse, errors::ApolloClientResult};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchResponse {
    pub app_id: String,
    pub cluster: String,
    pub namespace_name: String,
    pub configurations: HashMap<String, String>,
    pub release_key: String,
}
