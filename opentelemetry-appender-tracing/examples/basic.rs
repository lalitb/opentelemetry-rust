//! run with `$ cargo run --example basic

use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{
    logs::{Config, LoggerProvider},
    Resource,
};
use tracing::{error, Subscriber};
use tracing_subscriber::{prelude::*, Layer, Registry};

struct ConsoleLogger;
impl<S: Subscriber> Layer<S> for ConsoleLogger {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let metadata = event.metadata();
        println!("ConsoleLogger: {} - {}", metadata.level(), metadata.target());
    }
}

fn main() {
    let exporter = opentelemetry_stdout::LogExporter::default();
    let provider: LoggerProvider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "log-appender-tracing-example",
            )])),
        )
        .with_simple_exporter(exporter)
        .build();
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider);
    let console_layer = ConsoleLogger;
    let subscriber = Registry::default()
        .with(otel_layer)
        .with(console_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    error!(name: "my-event-name", target: "my-system", event_id = 20, user_name = "otel", user_email = "otel@opentelemetry.io");
    drop(provider);
}
