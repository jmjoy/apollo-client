use crate::meta::DEFAULT_CLUSTER_NAME;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenCreatedItem {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    comment: Option<Cow<'static, str>>,
    data_change_created_by: Cow<'static, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenUpdateItem {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    comment: Option<Cow<'static, str>>,
    #[builder(default, setter(strip_option))]
    data_change_created_by: Option<Cow<'static, str>>,
    data_change_last_modified_by: Cow<'static, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc, field_defaults(setter(into)))]
pub struct OpenRelease {
    release_title: Cow<'static, str>,
    #[builder(default, setter(strip_option))]
    release_comment: Option<Cow<'static, str>>,
    released_by: Cow<'static, str>,
}