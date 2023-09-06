//! run with `$ cargo run --example basic --all-features

use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_user_events_logs::{ExporterConfig, ReentrantLogProcessor};
use std::collections::HashMap;
use tracing::error;
use tracing_subscriber::prelude::*;

fn init_logger() -> LoggerProvider {
    let exporter_config = ExporterConfig {
        default_keyword: 1,
        keywords_map: HashMap::new(),
    };
    let reenterant_processor = ReentrantLogProcessor::new("test", None, exporter_config);
    LoggerProvider::builder()
        .with_log_processor(reenterant_processor)
        .build()
}

fn main() {
    // Example with tracing appender.
    let logger_provider = init_logger();
    let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
    tracing_subscriber::registry().with(layer).init();

    error!(
        user_name = "otel user",
        //event_name = "my-event-name",
        user_email = "otel@opentelemetry.io",
        "Login failed."
    );
}
