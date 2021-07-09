//! Configuration api metadata.

use crate::conf::requests::{FetchRequest, WatchRequest};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt::{self, Display},
};
use typed_builder::TypedBuilder;

pub(crate) const UNINITIALIZED_NOTIFICATION_ID: i32 = -1;

/// Notification for request and response, default notification_id is `-1`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc, field_defaults(setter(into)))]
pub struct Notification {
    namespace_name: Cow<'static, str>,
    #[builder(default_code = "UNINITIALIZED_NOTIFICATION_ID")]
    notification_id: i32,
}

impl Notification {
    #[inline]
    pub(crate) fn is_uninitialized(&self) -> bool {
        self.notification_id == UNINITIALIZED_NOTIFICATION_ID
    }

    // pub(crate) fn canonicalize(mut self) -> Self {
    //     self.namespace_name = if self.namespace_name.ends_with(".properties") {
    //         (&self.namespace_name[..self.namespace_name.len() - ".properties".len()])
    //             .to_string()
    //             .into()
    //     } else {
    //         self.namespace_name.into()
    //     };
    //     self
    // }

    pub(crate) fn update_notifications(older: &mut [Self], newer: &[Self]) {
        for newer_item in newer {
            for older_item in older.iter_mut() {
                if older_item.namespace_name == newer_item.namespace_name {
                    older_item.notification_id = newer_item.notification_id;
                }
            }
        }
    }

    pub(crate) fn create_fetch_requests(
        notifications: impl IntoIterator<Item = Self>,
        watch: &WatchRequest,
    ) -> Vec<FetchRequest> {
        notifications
            .into_iter()
            .map(|notification| FetchRequest::from_watch(watch, notification.namespace_name))
            .collect()
    }
}

implement_json_perform_response_for! { Vec<Notification> }

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

    /// Get the first ip of the machine match the cidr, such as '10.2.0.0/16'.
    #[cfg(feature = "host-ip")]
    #[cfg_attr(docsrs, doc(cfg(feature = "host-ip")))]
    HostCidr(cidr_utils::cidr::IpCidr),

    /// Specify your own IP address or other text.
    Custom(String),
}

impl IpValue {
    #[cfg(feature = "host-name")]
    fn get_host_name() -> &'static str {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                "test-host-name"
            } else {
                crate::utils::get_host_name()
            }
        }
    }

    #[cfg(feature = "host-ip")]
    fn get_all_addrs() -> &'static [std::net::IpAddr] {
        cfg_if::cfg_if! {
            if #[cfg(test)] {
                static TEST_IPS: [std::net::IpAddr; 2] = [
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 2, 0, 1)),
                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 3, 0, 1)),
                ];
                &TEST_IPS
            } else {
                crate::utils::get_all_addrs()
            }
        }
    }
}

impl Display for IpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "host-name")]
                IpValue::HostName => Cow::Borrowed(Self::get_host_name()),

                #[cfg(feature = "host-ip")]
                IpValue::HostIp => {
                    Self::get_all_addrs()
                        .get(0)
                        .map(|s| Cow::Owned(s.to_string()))
                        .unwrap_or(Cow::Borrowed("127.0.0.1"))
                }

                #[cfg(feature = "host-ip")]
                IpValue::HostCidr(cidr) => {
                    Self::get_all_addrs()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_new() {
        let notification = Notification::builder()
            .namespace_name("foo.properties")
            .build();
        assert_eq!(notification.namespace_name, "foo.properties");
        assert_eq!(notification.notification_id, -1);

        let notification = Notification::builder()
            .namespace_name("foo.yaml")
            .notification_id(10)
            .build();
        assert_eq!(notification.namespace_name, "foo.yaml");
        assert_eq!(notification.notification_id, 10);
    }

    #[test]
    fn test_update_notifications() {
        let mut notifications = [
            Notification::builder().namespace_name("foo").build(),
            Notification::builder()
                .namespace_name("bar")
                .notification_id(10)
                .build(),
        ];
        Notification::update_notifications(
            &mut notifications,
            &[Notification::builder()
                .namespace_name("foo")
                .notification_id(100)
                .build()],
        );
        assert_eq!(
            notifications,
            [
                Notification::builder()
                    .namespace_name("foo")
                    .notification_id(100)
                    .build(),
                Notification::builder()
                    .namespace_name("bar")
                    .notification_id(10)
                    .build(),
            ]
        );
    }

    #[test]
    fn test_ip_value_display() {
        #[cfg(feature = "host-name")]
        assert_eq!(IpValue::HostName.to_string(), "test-host-name");

        #[cfg(feature = "host-ip")]
        assert_eq!(IpValue::HostIp.to_string(), "10.2.0.1");

        #[cfg(feature = "host-ip")]
        assert_eq!(
            IpValue::HostCidr(cidr_utils::cidr::IpCidr::from_str("10.3.0.0/16").unwrap())
                .to_string(),
            "10.3.0.1"
        );

        assert_eq!(
            IpValue::Custom("custom-ip".to_owned()).to_string(),
            "custom-ip"
        );
    }
}
