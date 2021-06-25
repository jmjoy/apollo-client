# Apollo Client

[![Rustc Version](https://img.shields.io/badge/rustc-1.39+-lightgray.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)
[![Actions](https://github.com/jmjoy/apollo-client/workflows/Rust/badge.svg?branch=master&event=push)](https://github.com/jmjoy/apollo-client/actions?query=workflow%3ARust+branch%3Amaster+event%3Apush++)
[![Crate](https://img.shields.io/crates/v/apollo-client.svg)](https://crates.io/crates/apollo-client)
[![API](https://docs.rs/apollo-client/badge.svg)](https://docs.rs/apollo-client)
[![Lines](https://img.shields.io/tokei/lines/github/jmjoy/apollo-client)](https://github.com/jmjoy/apollo-client)
[![License](https://img.shields.io/crates/l/apollo-client)](https://github.com/jmjoy/apollo-client/blob/master/LICENSE)

Rust🦀 client for [Ctrip Apollo](https://github.com/ctripcorp/apollo).

Power by Rust `async/await`.

## Installation

With [cargo edit](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add -s --features full tokio
$ cargo add -s --features full apollo-client
```

## Support

- [x] Fetch config via config service.
- [ ] Fetch config via mata service.
- [x] Apollo open apis.

## Features

1. Not all features are default, you can read the `[features]` section of [Cargo.toml](https://github.com/jmjoy/apollo-client/blob/master/Cargo.toml) to know all the features.

1. The `xml` and `yaml` features aren't enable by default, if you have such kind namespace, you should add `features` in `Cargo.toml`, just like:

    ```toml
    apollo-client = { version = "0.6", features = ["yaml", "xml"] }
    ```

    Or simply enable all features:

    ```toml
    apollo-client = { version = "0.6", features = ["full"] }
    ```

## Usage

You can find some examples in [the examples directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

## License

[Unlicense](https://github.com/jmjoy/apollo-client/blob/master/LICENSE).
