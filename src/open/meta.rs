use crate::meta::DEFAULT_CLUSTER_NAME;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub(crate) env: Cow<'static, str>,
    pub(crate) app_id: Cow<'static, str>,
    pub(crate) cluster_name: Cow<'static, str>,
    pub(crate) namespace_name: Cow<'static, str>,
}

impl Namespace {
    pub fn new(
        env: impl Into<Cow<'static, str>>,
        app_id: impl Into<Cow<'static, str>>,
        namespace_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            env: env.into(),
            app_id: app_id.into(),
            cluster_name: DEFAULT_CLUSTER_NAME.into(),
            namespace_name: namespace_name.into(),
        }
    }

    pub fn cluster_name(mut self, cluster_name: impl Into<Cow<'static, str>>) -> Self {
        self.cluster_name = cluster_name.into();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCreatedItem {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    comment: Option<Cow<'static, str>>,
    data_change_created_by: Cow<'static, str>,
}

impl OpenCreatedItem {
    pub fn new(
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        data_change_created_by: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            comment: None,
            data_change_created_by: data_change_created_by.into(),
        }
    }

    pub fn comment(mut self, comment: impl Into<Cow<'static, str>>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenUpdateItem {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
    comment: Option<Cow<'static, str>>,
    data_change_created_by: Option<Cow<'static, str>>,
    data_change_last_modified_by: Cow<'static, str>,
}

impl OpenUpdateItem {
    pub fn new(
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        data_change_last_modified_by: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            comment: None,
            data_change_created_by: None,
            data_change_last_modified_by: data_change_last_modified_by.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    release_title: Cow<'static, str>,
    release_comment: Option<Cow<'static, str>>,
    released_by: Cow<'static, str>,
}

impl Release {
    pub fn new(
        release_title: impl Into<Cow<'static, str>>,
        released_by: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            release_title: release_title.into(),
            release_comment: None,
            released_by: released_by.into(),
        }
    }

    pub fn release_comment(mut self, release_comment: impl Into<Cow<'static, str>>) -> Self {
        self.release_comment = Some(release_comment.into());
        self
    }
}
