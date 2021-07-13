//! open api responses.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

macro_rules! struct_open_response_with_base_fields {
    (
        $(#[$enum_docs:meta])*
        $name:ident,
        {
            $( ($i:ident, $t:ty) ,)*
        }
    ) => {
        $(#[$enum_docs])*
        #[derive(Clone, Debug, Serialize, Deserialize)]
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

macro_rules! struct_open_response_with_namespace_fields {
    (
        $(#[$enum_docs:meta])*
        $name:ident,
        {
            $( ($i:ident, $t:ty) ,)*
        }
    ) => {
        struct_open_response_with_base_fields! {
            $(#[$enum_docs])*
            $name,
            {
                (app_id, String),
                (cluster_name, String),
                (namespace_name, String),
            }
        }
    };
}

/// Response for [crate::open::requests::OpenEnvClusterRequest].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenEnvClusterResponse {
    pub env: String,
    pub clusters: Vec<String>,
}

implement_json_perform_response_for! { Vec<OpenEnvClusterResponse> }

struct_open_response_with_base_fields! {
    /// Response for [crate::open::requests::OpenAppRequest].
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

implement_json_perform_response_for! { Vec<OpenAppResponse> }

struct_open_response_with_namespace_fields! {
    /// Response for [crate::open::requests::OpenNamespaceRequest].
    OpenNamespaceResponse,
    {
        (comment, Option<String>),
        (format, String),
        (is_public, bool),
        (items, Vec<OpenItemResponse>),
    }
}

implement_json_perform_response_for! { Vec<OpenNamespaceResponse> }

struct_open_response_with_base_fields! {
    /// Response for [crate::open::requests::OpenCreateItemRequest].
    OpenItemResponse,
    {
        (key, String),
        (value, String),
        (comment, Option<String>),
    }
}

implement_json_perform_response_for! { OpenItemResponse }
implement_json_perform_response_for! { Vec<OpenItemResponse> }

struct_open_response_with_namespace_fields! {
    /// Response for [crate::open::requests::OpenPublishNamespaceRequest].
    OpenPublishResponse,
    {
        (name, String),
        (configurations, HashMap<String, String>),
        (comment, Option<String>),
    }
}

implement_json_perform_response_for! { OpenPublishResponse }

struct_open_response_with_base_fields! {
    /// Response for [crate::open::requests::OpenClusterRequest].
    OpenClusterResponse,
    {
        (name, String),
        (app_id, String),
    }
}

implement_json_perform_response_for! { OpenClusterResponse }
