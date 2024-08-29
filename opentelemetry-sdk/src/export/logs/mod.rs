//! Log exporters
use crate::logs::LogRecord;
use crate::Resource;
use async_trait::async_trait;
#[cfg(feature = "logs_level_enabled")]
use opentelemetry::logs::Severity;
use opentelemetry::{
    logs::{LogError, LogResult},
    InstrumentationLibrary,
};
use std::fmt::Debug;

/// A batch of log records to be exported by a `LogExporter`.
#[derive(Debug)]
pub struct LogBatch<'a> {
    data: &'a [(&'a LogRecord, &'a InstrumentationLibrary)],
}

impl<'a> LogBatch<'a> {
    pub fn new(data: &'a [(&'a LogRecord, &'a InstrumentationLibrary)]) -> LogBatch<'a> {
        LogBatch { data }
    }
}

impl LogBatch<'_> {
    /// iterator
    pub fn iter(&self) -> impl Iterator<Item = (&LogRecord, &InstrumentationLibrary)> {
        self.data
            .iter()
            .map(|(record, library)| (*record, *library))
    }
}

/// `LogExporter` defines the interface that log exporters should implement.
#[async_trait]
pub trait LogExporter: Send + Sync + Debug {
    /// Exports a batch of [`LogRecord`, `InstrumentationLibrary`].
    async fn export(&mut self, batch: LogBatch<'_>) -> LogResult<()>;
    /// Shuts down the exporter.
    fn shutdown(&mut self) {}
    #[cfg(feature = "logs_level_enabled")]
    /// Chek if logs are enabled.
    fn event_enabled(&self, _level: Severity, _target: &str, _name: &str) -> bool {
        // By default, all logs are enabled
        true
    }
    /// Set the resource for the exporter.
    fn set_resource(&mut self, _resource: &Resource) {}
}

/// Describes the result of an export.
pub type ExportResult = Result<(), LogError>;
