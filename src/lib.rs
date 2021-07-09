#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![forbid(non_ascii_idents, unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*!
[![Rustc Version](https://img.shields.io/badge/rustc-1.39+-lightgray.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)
[![Actions](https://github.com/jmjoy/apollo-client/workflows/Rust/badge.svg?branch=master&event=push)](https://github.com/jmjoy/apollo-client/actions?query=workflow%3ARust+branch%3Amaster+event%3Apush++)
[![Crate](https://img.shields.io/crates/v/apollo-client.svg)](https://crates.io/crates/apollo-client)
[![API](https://docs.rs/apollo-client/badge.svg)](https://docs.rs/apollo-client)
[![Lines](https://img.shields.io/tokei/lines/github/jmjoy/apollo-client)](https://github.com/jmjoy/apollo-client)
[![License](https://img.shields.io/crates/l/apollo-client)](https://github.com/jmjoy/apollo-client/blob/master/LICENSE)

Rust🦀 client for [Ctrip Apollo](https://github.com/ctripcorp/apollo).

Power by Rust `async/await`.

# Installation

With [cargo edit](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add -s --features full tokio
$ cargo add -s --features full apollo-client
```

# Support

- [x] Fetch config via config service.
- [ ] Fetch config via mata service.
- [ ] Authorization for configuration api.
- [x] Apollo open apis.

# Features

- **default**: Include **conf**, no **open**.
- **conf**: Apollo configuration apis.
- **open**: Apollo open platformm apis.
- **full**: All features.

# Usage

Simple fetch configuration:

```rust
use apollo_client::{
    conf::{meta::IpValue, requests::CachedFetchRequest, ApolloConfClientBuilder},
    errors::ApolloClientResult,
};
use ini::Properties;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create configuration client.
    let client =
        ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
            .build()?;

    // Request apollo cached configuration api.
    let configuration: Properties = client
        .execute(
            CachedFetchRequest::builder()
                .app_id("SampleApp")
                .namespace_name("application.json")
                .ip(IpValue::HostName)
                .build(),
        )
        .await?;

    // Get the content of configuration.
    let content = configuration.get("content");
    dbg!(content);

    Ok(())
}
```

Watch configuration and fetch when changed:

```rust
use apollo_client::conf::{meta::IpValue, requests::WatchRequest, ApolloConfClientBuilder};
use cidr_utils::cidr::IpCidr;
use futures_util::{pin_mut, stream::StreamExt};
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create configuration client.
    let client =
        ApolloConfClientBuilder::new_via_config_service(Url::parse("http://localhost:8080")?)?
            .build()?;

    // Request apollo notification api, and fetch configuration when notified.
    let stream = client.watch(
        WatchRequest::builder()
            .app_id("SampleApp")
            .namespace_names([
                "application.properties".into(),
                "application.json".into(),
                "application.yml".into(),
            ])
            .ip(IpValue::HostCidr(IpCidr::from_str("172.16.0.0/16")?))
            .build(),
    );

    pin_mut!(stream);

    // These is a dead loop, `next()` is returned when configuration is changed.
    while let Some(response) = stream.next().await {
        let responses = response?;
        for response in responses {
            let _ = dbg!(response);
        }
    }

    Ok(())
}
```

Call open platform api to fetch app infos:

```rust
use std::error::Error;
use apollo_client::open::OpenApiClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create open platform api client.
    let client = OpenApiClientBuilder::new(
        "http://127.0.0.1:8070/".parse()?,
        "391cc4053f8cce2e452a0e6db8925bbba503f434",
    )?
    .build()?;

    // Execute app fetching request.
    let responses = client
        .execute(
            OpenAppRequest::builder()
                .app_ids(vec!["SampleApp".into()])
                .build(),
        )
        .await?;

    dbg!(responses);

    Ok(())
}
```

You can find more examples in [the examples directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

# License

[Unlicense](https://github.com/jmjoy/apollo-client/blob/master/LICENSE).
*/

#[macro_use]
pub mod meta;
#[cfg(feature = "conf")]
#[cfg_attr(docsrs, doc(cfg(feature = "conf")))]
pub mod conf;
pub mod errors;
#[cfg(feature = "open")]
#[cfg_attr(docsrs, doc(cfg(feature = "open")))]
pub mod open;
pub mod utils;
