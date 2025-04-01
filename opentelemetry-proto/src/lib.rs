#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! This crate contains generated files from [opentelemetry-proto](https://github.com/open-telemetry/opentelemetry-proto)
//! repository and transformation between types from generated files and types defined in [opentelemetry](https://github.com/open-telemetry/opentelemetry-rust/tree/main/opentelemetry)
//!
//!  The rs files are generated using [tonic](https://github.com/hyperium/tonic) and [prost](https://github.com/tokio-rs/prost).
//!
//! # Feature flags
//! `Opentelemetry-proto` includes a set of feature flags to avoid pull in unnecessary dependencies.
//! The following is the full list of currently supported features:
//!
//! ## Signals
//! - `trace`: generate types that used in traces. Currently supports `gen-tonic`.
//! - `metrics`: generate types that used in metrics. Currently supports `gen-tonic`.
//! - `logs`: generate types that used in logs. Currently supports `gen-tonic`.
//! - `zpages`: generate types that used in zPages. Currently only tracez related types will be generated. Currently supports `gen-tonic`.
//!
//! ## Crates used to generate files
//! - `gen-tonic`: adding tonic transport to the generated files. This is the default feature.
//!
//! ## Misc
//! - `full`: enabled all features above.
//!
//! By default, no feature is enabled.

// proto mod contains file generated by protobuf or other build tools.
// we shouldn't manually change it. Thus skip format and lint check.
#[rustfmt::skip]
#[allow(warnings)]
#[doc(hidden)]
mod proto;

pub use proto::tonic;

pub mod transform;
