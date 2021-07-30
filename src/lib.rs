#![warn(rust_2018_idioms, clippy::dbg_macro, clippy::print_stdout)]
#![forbid(non_ascii_idents, unsafe_code)]
#![cfg_attr(docsrs, allow(rustdoc::invalid_rust_codeblocks))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

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
