//! run with `$ cargo run --example basic --all-features

use opentelemetry_stdout::{LogData, LogExporter, LogExporterBuilder};
use opentelemetry_tokio_logging::{layer};
use tracing::{info, error};
use opentelemetry_sdk::logs::{LoggerProvider};
use opentelemetry_api::global;
use tracing_subscriber::prelude::*;

fn main() {
    let exporter  = opentelemetry_stdout::LogExporter::default();
    let provider: LoggerProvider = LoggerProvider::builder().with_simple_exporter(exporter).build();
    let _ = global::set_logger_provider(provider.clone());
    let layer  = layer::OpenTelemetryLogsLayer::new(provider.clone());
    tracing_subscriber::registry().with(layer).init();

    info!("test");
    drop(provider);
    global::shutdown_logger_provider();
}

