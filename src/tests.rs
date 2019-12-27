use super::*;

#[test]
fn test_ip_value_deserialize() -> ApolloClientResult<()> {
    assert_eq!(serde_json::to_string(&IpValue::HostName)?, r#""HostName""#);
    assert_eq!(
        serde_json::to_string(&IpValue::Custom("127.0.0.2"))?,
        r#"{"Custom":"127.0.0.2"}"#
    );
    assert_eq!(
        serde_json::from_str::<IpValue>(r#""HostName""#)?,
        IpValue::HostName
    );
    assert_eq!(
        serde_json::from_str::<IpValue>(r#"{"Custom":"127.0.0.2"}"#)?,
        IpValue::Custom("127.0.0.2")
    );
    Ok(())
}

#[test]
fn test_client_get_config_url() -> ApolloClientResult<()> {
    let client_config = ClientConfig {
        app_id: "test_app_id",
        ..Default::default()
    };
    test_client_get_config_url_common(
        client_config,
        "http://localhost:8080/configs/test_app_id/default/test_namespace",
    )?;
    Ok(())
}

#[test]
fn test_client_get_config_url_2() -> ApolloClientResult<()> {
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
fn test_client_get_config_url_3() -> ApolloClientResult<()> {
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

fn test_client_get_config_url_common(
    client_config: ClientConfig,
    expect: &str,
) -> ApolloClientResult<()> {
    let client = Client::with_config(&client_config);
    let url = client.get_config_url("test_namespace", None)?;
    assert_eq!(&url, expect);
    Ok(())
}

#[test]
fn test_client_get_listen_url() -> ApolloClientResult<()> {
    test_client_get_listen_url_common(
        &initialize_notifications(&[]),
        "http://localhost:8080/notifications/v2?appId=test_app_id&cluster=default",
    )?;
    Ok(())
}

#[test]
fn test_client_get_listen_url_2() -> ApolloClientResult<()> {
    test_client_get_listen_url_common(
        &initialize_notifications(&["test-namespace"]),
        "http://localhost:8080/notifications/v2?appId=test_app_id&cluster=default&notifications=%5B%7B%22namespaceName%22%3A%22test-namespace%22%2C%22notificationId%22%3A-1%7D%5D"
    )?;
    Ok(())
}

#[test]
fn test_client_get_listen_url_3() -> ApolloClientResult<()> {
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
) -> ApolloClientResult<()> {
    let client_config = ClientConfig {
        app_id: "test_app_id",
        ..Default::default()
    };
    let client = Client::with_config(&client_config);
    let url = client.get_listen_url(notifications)?;
    assert_eq!(&url, expect);
    Ok(())
}
