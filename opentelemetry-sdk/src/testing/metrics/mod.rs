//! In-memory metrics exporter for testing purposes.

/// The `in_memory_exporter` module provides in-memory metrics functionalities.
/// For detailed usage and examples, see `in_memory_exporter`.
pub mod in_memory_exporter;
pub use in_memory_exporter::{InMemoryMetricsExporter, InMemoryMetricsExporterBuilder};

#[doc(hidden)]
pub mod metric_reader;
pub use metric_reader::TestMetricReader;
