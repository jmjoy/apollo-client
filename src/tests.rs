use super::*;
use std::error::Error;

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
fn test_client_get_config_url2() -> ApolloClientResult<()> {
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
fn test_client_get_config_url3() -> ApolloClientResult<()> {
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
    let client = Client::new_with_config(&client_config);
    let url = client.get_config_url("test_namespace", None)?;
    assert_eq!(&url, expect);
    Ok(())
}
