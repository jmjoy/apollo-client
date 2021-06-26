use chrono::{DateTime, Local};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::SystemTime;

macro_rules! open_response_with_base_fields {
    ($name:ident, { $( ($i:ident, $t:ty) ,)* }) => {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(pub $i : $t,)*
            pub data_change_created_by: String,
            pub data_change_last_modified_by: String,
            pub data_change_created_by_display_name: Option<String>,
            pub data_change_last_modified_by_display_name: Option<String>,
            pub data_change_created_time: DateTime<Local>,
            pub data_change_last_modified_time: DateTime<Local>,
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenEnvClusterResponse {
    pub env: String,
    pub clusters: Vec<String>,
}

implement_json_perform_response!(Vec<OpenEnvClusterResponse>);

open_response_with_base_fields! {
    OpenAppResponse,
    {
        (name, String),
        (app_id, String),
        (org_id, String),
        (org_name, String),
        (owner_name, String),
        (owner_email, String),
    }
}

implement_json_perform_response!(Vec<OpenAppResponse>);

open_response_with_base_fields! {
    OpenNamespaceResponse,
    {
        (app_id, String),
        (cluster_name, String),
        (namespace_name, String),
        (comment, Option<String>),
        (format, String),
        (is_public, bool),
        (items, Vec<OpenItemResponse>),
    }
}

implement_json_perform_response!(Vec<OpenNamespaceResponse>);

open_response_with_base_fields! {
    OpenItemResponse,
    {
        (key, String),
        (value, String),
        (comment, Option<String>),
    }
}

implement_json_perform_response!(Vec<OpenItemResponse>);
