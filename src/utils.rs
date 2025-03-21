//! Useful utilities.

#[cfg(feature = "host-name")]
#[allow(dead_code)]
pub(crate) fn get_host_name() -> &'static str {
    use once_cell::sync::OnceCell;

    static HOST_NAME: OnceCell<String> = OnceCell::new();
    HOST_NAME.get_or_init(|| {
        hostname::get()
            .ok()
            .and_then(|hostname| hostname.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
    })
}

#[cfg(feature = "host-ip")]
#[allow(dead_code)]
pub(crate) fn get_all_addrs() -> &'static [std::net::IpAddr] {
    use once_cell::sync::OnceCell;
    use systemstat::{data::IpAddr, platform::common::Platform, System};

    static ALL_ADDRS: OnceCell<Vec<std::net::IpAddr>> = OnceCell::new();
    ALL_ADDRS.get_or_init(|| {
        System::new()
            .networks()
            .ok()
            .map(|networks| {
                networks
                    .values()
                    .flat_map(|network| {
                        network
                            .addrs
                            .iter()
                            .filter_map(|network_addr| match network_addr.addr {
                                IpAddr::V4(addr) => {
                                    if addr.is_loopback() {
                                        None
                                    } else {
                                        Some(std::net::IpAddr::V4(addr))
                                    }
                                }
                                IpAddr::V6(addr) => {
                                    if addr.is_loopback() {
                                        None
                                    } else {
                                        Some(std::net::IpAddr::V6(addr))
                                    }
                                }
                                _ => None,
                            })
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

/// Canonicalize the namespace. Just add `.properties` to the end of the namespace which not end
/// with `.properties` or `.xml` or `.json` or `.yaml` or `.yml` or `.txt`.
///
/// # Examples
///
/// ```rust
/// use apollo_client::utils::canonicalize_namespace;
/// assert_eq!(canonicalize_namespace("foo"), "foo.properties");
/// assert_eq!(canonicalize_namespace("foo.yaml"), "foo.yaml");
/// ```
pub fn canonicalize_namespace(namespace: &str) -> String {
    if namespace.ends_with(".properties")
        || namespace.ends_with(".xml")
        || namespace.ends_with(".json")
        || namespace.ends_with(".yaml")
        || namespace.ends_with(".yml")
        || namespace.ends_with(".txt")
    {
        namespace.to_string()
    } else {
        format!("{}.properties", namespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_namespace() {
        assert_eq!(canonicalize_namespace("foo.properties"), "foo.properties");
        assert_eq!(canonicalize_namespace("foo.xml"), "foo.xml");
        assert_eq!(canonicalize_namespace("foo.yaml"), "foo.yaml");
        assert_eq!(canonicalize_namespace("foo.yml"), "foo.yml");
        assert_eq!(canonicalize_namespace("foo.json"), "foo.json");
        assert_eq!(canonicalize_namespace("foo.txt"), "foo.txt");
        assert_eq!(canonicalize_namespace("foo"), "foo.properties");
    }
}
