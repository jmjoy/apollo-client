//! open api metadata.

use serde::{Deserialize, Serialize};

/// Item for [crate::open::requests::OpenCreateItemRequest].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCreatedItem {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub data_change_created_by: String,
}

impl Default for OpenCreatedItem {
    fn default() -> Self {
        OpenCreatedItem {
            key: "".to_string(),
            value: "".to_string(),
            comment: None,
            data_change_created_by: "".to_string(),
        }
    }
}

/// Item for [crate::open::requests::OpenUpdateItemRequest].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenUpdateItem {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub data_change_created_by: Option<String>,
    pub data_change_last_modified_by: String,
}

impl Default for OpenUpdateItem {
    fn default() -> Self {
        OpenUpdateItem {
            key: "".to_string(),
            value: "".to_string(),
            comment: None,
            data_change_created_by: None,
            data_change_last_modified_by: "".to_string(),
        }
    }
}

/// Item for [crate::open::requests::OpenPublishNamespaceRequest].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenRelease {
    pub release_title: String,
    pub release_comment: Option<String>,
    pub released_by: String,
}

impl Default for OpenRelease {
    fn default() -> Self {
        OpenRelease {
            release_title: "".to_string(),
            release_comment: None,
            released_by: "".to_string(),
        }
    }
}
