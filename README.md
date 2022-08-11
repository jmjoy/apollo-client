# Apollo Client

[![Actions](https://github.com/jmjoy/apollo-client/workflows/Rust/badge.svg?branch=master&event=push)](https://github.com/jmjoy/apollo-client/actions?query=workflow%3ARust+branch%3Amaster+event%3Apush++)
[![Crate](https://img.shields.io/crates/v/apollo-client.svg)](https://crates.io/crates/apollo-client)
[![API](https://docs.rs/apollo-client/badge.svg)](https://docs.rs/apollo-client)
[![Lines](https://img.shields.io/tokei/lines/github/jmjoy/apollo-client)](https://github.com/jmjoy/apollo-client)
[![License](https://img.shields.io/crates/l/apollo-client)](https://github.com/jmjoy/apollo-client/blob/master/LICENSE)

[Ctrip Apollo](https://github.com/ctripcorp/apollo) client for Rust🦀.

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
- [x] Authorization for configuration api.
- [x] Apollo open apis.

## Features

- **host**: IpValue HostName, HostIp and HostCidr options, enable by default.
- **conf**: Apollo configuration apis, enable by default.
- **open**: Apollo open platform apis.
- **full**: All features.

## Usage

You can find more examples in [the example directory](https://github.com/jmjoy/apollo-client/tree/master/examples).

## License

[Unlicense](https://github.com/jmjoy/apollo-client/blob/master/LICENSE).
