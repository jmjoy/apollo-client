//! Configuration api response.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response of [crate::conf::requests::FetchRequest].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchResponse {
    pub app_id: String,
    pub cluster: String,
    pub namespace_name: String,
    pub configurations: HashMap<String, String>,
    pub release_key: String,
}

implement_json_perform_response_for! { FetchResponse }
