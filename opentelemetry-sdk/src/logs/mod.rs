//! # OpenTelemetry Log SDK

mod config;
mod log_emitter;
mod log_processor;

pub use config::Config;
pub use log_emitter::{Builder, Logger, LoggerProvider};
pub use log_processor::{
    BatchConfig, BatchLogProcessor, BatchLogProcessorBuilder, BatchMessage, LogProcessor,
    SimpleLogProcessor,
};
pub use opentelemetry_api::logs::Severity;
use std::fmt;

/// Interface for checking if a log level is enabled.
pub trait EventEnabled: Send + Sync + fmt::Debug {
    /// Generate a new `TraceId`
    fn is_enabled(&self, _name: &str, _level: Severity) -> bool;
}

/// Default [`EventEnabled`] implementation.
///
/// return true for all log levels.
#[derive(Clone, Debug, Default)]
pub struct DefaultEventEnabled {
    _private: (),
}

impl EventEnabled for DefaultEventEnabled {
    fn is_enabled(&self, _name: &str, _level: Severity) -> bool {
        true
    }
}
