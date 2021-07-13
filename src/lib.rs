#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![forbid(non_ascii_idents, unsafe_code)]
#![cfg_attr(docsrs, allow(rustdoc::invalid_rust_codeblocks))]
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

- **host**: IpValue HostName, HostIp and HostCidr options, enable by default.
- **conf**: Apollo configuration apis, enable by default.
- **open**: Apollo open platform apis.
- **full**: All features.

# Usage

You can find more examples in [the example directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

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
