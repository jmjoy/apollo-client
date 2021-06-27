use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::conf::requests::{FetchRequest, Watch};
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        let notification = Notification::new("foo.properties");
        assert_eq!(notification.namespace_name, "foo");
        assert_eq!(notification.notification_id, -1);

        let notification = Notification::new("foo.yaml").notification_id(10);
        assert_eq!(notification.namespace_name, "foo.yaml");
        assert_eq!(notification.notification_id, 10);
    }

    #[test]
    fn test_update_notifications() {
        let mut notifications = [
            Notification::new("foo"),
            Notification::new("bar").notification_id(10),
        ];
        Notification::update_notifications(
            &mut notifications,
            &[Notification::new("foo").notification_id(100)],
        );
        assert_eq!(
            notifications,
            [
                Notification::new("foo").notification_id(100),
                Notification::new("bar").notification_id(10),
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
