#[cfg(feature = "host-name")]
pub(crate) fn get_hostname() -> &'static str {
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
pub(crate) fn get_all_addrs() -> &'static [String] {
    use once_cell::sync::OnceCell;
    use systemstat::{data::IpAddr, platform::common::Platform, System};

    static ALL_ADDRS: OnceCell<Vec<String>> = OnceCell::new();
    ALL_ADDRS.get_or_init(|| {
        System::new()
            .networks()
            .ok()
            .map(|networks| {
                networks
                    .values()
                    .map(|network| {
                        network
                            .addrs
                            .iter()
                            .filter_map(|network_addr| match network_addr.addr {
                                IpAddr::V4(addr) => Some(addr.to_string()),
                                IpAddr::V6(addr) => Some(addr.to_string()),
                                _ => None,
                            })
                    })
                    .flatten()
                    .collect()
            })
            .unwrap_or(Vec::new())
    })
}
