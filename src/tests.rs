use super::*;

#[test]
fn test_ip_value_deserialize() -> ClientResult<()> {
    #[cfg(feature = "host-name")]
    assert_eq!(
        serde_json::to_string::<IpValue<&str>>(&IpValue::HostName)?,
        r#""host-name""#
    );

    #[cfg(feature = "host-ip")]
    assert_eq!(
        serde_json::to_string(&IpValue::HostIpRegex(r"^127\."))?,
        r#"{"host-ip-regex":"^127\\."}"#
    );

    assert_eq!(
        serde_json::to_string(&IpValue::Custom("127.0.0.2"))?,
        r#"{"custom":"127.0.0.2"}"#
    );

    #[cfg(feature = "host-name")]
    assert_eq!(
        serde_json::from_str::<IpValue<&str>>(r#""host-name""#)?,
        IpValue::HostName
    );

    #[cfg(feature = "host-ip")]
    assert_eq!(
        serde_json::from_str::<IpValue<String>>(r#"{"host-ip-regex":"^127\\."}"#)?,
        IpValue::HostIpRegex(r"^127\.".to_owned())
    );

    assert_eq!(
        serde_json::from_str::<IpValue<&str>>(r#"{"custom":"127.0.0.2"}"#)?,
        IpValue::Custom("127.0.0.2")
    );

    assert_eq!(
        serde_json::from_str::<IpValue<String>>(r#"{"custom":"127.0.0.2"}"#)?,
        IpValue::Custom("127.0.0.2".to_string())
    );

    Ok(())
}

#[test]
fn test_ip_value() {
    #[cfg(feature = "host-ip")]
    assert_eq!(
        IpValue::HostIpRegex(r"^127\.0\.0\.1$").to_str(),
        "127.0.0.1"
    );

    assert_eq!(IpValue::Custom("test-host-name").to_str(), "test-host-name");

    assert_eq!(
        IpValue::Custom("test-host-name".to_string()).to_str(),
        "test-host-name"
    );
}

#[test]
fn test_client_get_config_url() -> ClientResult<()> {
    let client_config = ClientConfig {
        app_id: "test_app_id",
        ..Default::default()
    };
    test_client_get_config_url_common(
        client_config,
        "http://localhost:8080/configs/test_app_id/default/test_namespace",
    )?;

    let client_config = ClientConfig {
        app_id: "test_app_id".to_string(),
        ..Default::default()
    };
    let client = Client::new(client_config);
    let url = client.get_config_url("test_namespace", None, None)?;
    assert_eq!(
        &url,
        "http://localhost:8080/configs/test_app_id/default/test_namespace"
    );

    Ok(())
}

#[test]
fn test_client_get_config_url_2() -> ClientResult<()> {
    let client_config = ClientConfig {
        app_id: "test_app_id",
        ip: Some(IpValue::Custom("127.0.0.2")),
        ..Default::default()
    };
    test_client_get_config_url_common(
        client_config,
        "http://localhost:8080/configs/test_app_id/default/test_namespace?ip=127.0.0.2",
    )?;
    Ok(())
}

#[test]
fn test_client_get_config_url_3() -> ClientResult<()> {
    let client_config = ClientConfig {
        app_id: "test_app_id",
        ip: Some(IpValue::Custom("???")),
        ..Default::default()
    };
    test_client_get_config_url_common(
        client_config,
        "http://localhost:8080/configs/test_app_id/default/test_namespace?ip=%3F%3F%3F",
    )?;
    Ok(())
}

#[test]
fn test_client_get_config_url_4() -> ClientResult<()> {
    let client_config: ClientConfig<&'static str, &'static [&'static str]> = ClientConfig {
        app_id: "test_app_id",
        ip: Some(IpValue::Custom("???")),
        ..Default::default()
    };
    let client = Client::new(client_config);
    let url = client.get_config_url("test_namespace", Some("test-release"), None)?;
    assert_eq!(&url, "http://localhost:8080/configs/test_app_id/default/test_namespace?releaseKey=test-release&ip=%3F%3F%3F");
    Ok(())
}

#[test]
fn test_client_get_config_url_5() -> ClientResult<()> {
    let client_config: ClientConfig<&'static str, &'static [&'static str]> = ClientConfig {
        app_id: "test_app_id",
        ..Default::default()
    };
    let client = Client::new(client_config);
    let url = client.get_config_url(
        "test_namespace",
        Some("test-release"),
        Some(&[("noAudit", "1")]),
    )?;
    assert_eq!(&url, "http://localhost:8080/configs/test_app_id/default/test_namespace?releaseKey=test-release&noAudit=1");
    Ok(())
}

#[test]
fn test_client_get_config_url_6() -> ClientResult<()> {
    let client_config: ClientConfig<&'static str, &'static [&'static str]> = ClientConfig {
        app_id: "test_app_id",
        ip: Some(IpValue::Custom("127.0.0.1")),
        ..Default::default()
    };
    let client = Client::new(client_config);
    let url = client.get_config_url("test_namespace", None, Some(&[("noAudit", "1")]))?;
    assert_eq!(
        &url,
        "http://localhost:8080/configs/test_app_id/default/test_namespace?ip=127.0.0.1&noAudit=1"
    );
    Ok(())
}

fn test_client_get_config_url_common<'a>(
    client_config: ClientConfig<&'a str, Vec<&'a str>>,
    expect: &str,
) -> ClientResult<()> {
    let client = Client::new(client_config);
    let url = client.get_config_url("test_namespace", None, None)?;
    assert_eq!(&url, expect);
    Ok(())
}

#[test]
fn test_client_get_listen_url() -> ClientResult<()> {
    test_client_get_listen_url_common(
        &initialize_notifications::<&str>(&[]),
        "http://localhost:8080/notifications/v2?appId=test_app_id&cluster=default",
    )?;
    Ok(())
}

#[test]
fn test_client_get_listen_url_2() -> ClientResult<()> {
    test_client_get_listen_url_common(
        &initialize_notifications(&["test-namespace"]),
        "http://localhost:8080/notifications/v2?appId=test_app_id&cluster=default&notifications=%5B%7B%22namespaceName%22%3A%22test-namespace%22%2C%22notificationId%22%3A-1%7D%5D"
    )?;
    Ok(())
}

#[test]
fn test_client_get_listen_url_3() -> ClientResult<()> {
    let mut notifications = initialize_notifications(&["test-namespace-2"]);
    notifications[0].notification_id = 100;
    test_client_get_listen_url_common(
        &notifications,
        "http://localhost:8080/notifications/v2?appId=test_app_id&cluster=default&notifications=%5B%7B%22namespaceName%22%3A%22test-namespace-2%22%2C%22notificationId%22%3A100%7D%5D"
    )?;
    Ok(())
}

fn test_client_get_listen_url_common(
    notifications: &Notifications,
    expect: &str,
) -> ClientResult<()> {
    let client_config: ClientConfig<&'static str, &'static [&'static str]> = ClientConfig {
        app_id: "test_app_id",
        ..Default::default()
    };
    let client = Client::new(client_config);
    let url = client.get_listen_url(notifications)?;
    assert_eq!(&url, expect);
    Ok(())
}

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

#[test]
fn test_infer_namespace_kind() {
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.properties"),
        NamespaceKind::Properties
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.xml"),
        NamespaceKind::Xml
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.yaml"),
        NamespaceKind::Yaml
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.yml"),
        NamespaceKind::Yaml
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.json"),
        NamespaceKind::Json
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo.txt"),
        NamespaceKind::Txt
    );
    assert_eq!(
        NamespaceKind::infer_namespace_kind("foo"),
        NamespaceKind::Properties
    );
}
