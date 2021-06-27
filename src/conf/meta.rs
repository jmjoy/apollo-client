use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    conf::requests::{FetchRequest, Watch},
    utils,
};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    namespace_name: Cow<'static, str>,
    notification_id: i32,
}

impl Notification {
    pub fn new(namespace_name: impl Into<Cow<'static, str>>) -> Notification {
        let mut namespace_name = namespace_name.into();

        if namespace_name.ends_with(".properties") {
            namespace_name = (&namespace_name[..namespace_name.len() - ".properties".len()])
                .to_string()
                .into();
        }

        Self {
            namespace_name,
            notification_id: -1,
        }
    }

    pub fn notification_id(mut self, notification_id: i32) -> Notification {
        self.notification_id = notification_id;
        self
    }

    pub(crate) fn update_notifications(older: &mut [Notification], newer: &[Notification]) {
        for newer_item in newer {
            for older_item in older.iter_mut() {
                if older_item.namespace_name == newer_item.namespace_name {
                    older_item.notification_id = newer_item.notification_id;
                }
            }
        }
    }

    pub(crate) fn create_fetch_requests(
        notifications: impl IntoIterator<Item = Notification>,
        watch: &Watch,
    ) -> Vec<FetchRequest> {
        notifications
            .into_iter()
            .map(|notification| FetchRequest::from_watch(watch, notification.namespace_name))
            .collect()
    }
}

implement_json_perform_response!(Vec<Notification>);

/// Apollo config api `ip` param value.
#[derive(Debug, Clone, PartialEq)]
pub enum IpValue {
    /// Get the hostname of the machine.
    #[cfg(feature = "host-name")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-name")))]
    HostName,

    /// Get the first ip of the machine generally.
    #[cfg(feature = "host-ip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-ip")))]
    HostIp,

    /// Get the first ip of the machine match the prefix, such as `^10\.2\.`.
    #[cfg(feature = "host-ip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-ip")))]
    HostIpRegex(cidr_utils::cidr::IpCidr),

    /// Specify your own IP address or other text.
    Custom(String),
}

impl Display for IpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "host-name")]
                IpValue::HostName => Cow::Borrowed(utils::get_hostname()),

                #[cfg(feature = "host-ip")]
                IpValue::HostIp => {
                    utils::get_all_addrs()
                        .iter()
                        .nth(0)
                        .map(|s| Cow::Owned(s.to_string()))
                        .unwrap_or(Cow::Borrowed("127.0.0.1"))
                }

                #[cfg(feature = "host-ip")]
                IpValue::HostIpRegex(cidr) => {
                    utils::get_all_addrs()
                        .iter()
                        .find(|addr| cidr.contains(**addr))
                        .map(|s| Cow::Owned(s.to_string()))
                        .unwrap_or(Cow::Borrowed("127.0.0.1"))
                }

                IpValue::Custom(s) => Cow::Borrowed(s.as_ref()),
            }
        )
    }
}
