#![cfg(unix)]

use integration_test_runner::logs_asserter::{read_logs_from_json, LogsAsserter};
use integration_test_runner::Protocol;
use log::{info, Level, Log, SetLoggerError};
use opentelemetry::logs::LogError;
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::{logs as sdklogs, runtime, Resource};
use std::error::Error;
use std::fs::File;
use std::os::unix::fs::MetadataExt;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Option<Box<dyn Log>>> = Mutex::new(None);
}

fn init_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError> {
    let mut global_logger = LOGGER.lock().unwrap();
    if global_logger.is_none() {
        *global_logger = Some(logger);
        log::set_logger(global_logger.as_ref().unwrap().as_ref())
    } else {
        Ok(())
    }
}

trait ExporterBuilder: Send + Sync {
    fn build(self: Box<Self>) -> Result<sdklogs::LoggerProvider, LogError>;
}

impl ExporterBuilder for opentelemetry_otlp::TonicExporterBuilder {
    fn build(self: Box<Self>) -> Result<sdklogs::LoggerProvider, LogError> {
        opentelemetry_otlp::new_pipeline()
            .logging()
            .with_exporter(*self)
            .with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "logs-integration-test",
            )]))
            .install_batch(runtime::Tokio)
    }
}

impl ExporterBuilder for opentelemetry_otlp::HttpExporterBuilder {
    fn build(self: Box<Self>) -> Result<sdklogs::LoggerProvider, LogError> {
        opentelemetry_otlp::new_pipeline()
            .logging()
            .with_exporter(*self)
            .with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "logs-integration-test",
            )]))
            .install_batch(runtime::Tokio)
    }
}

fn init_logs(protocol: &Protocol) -> Result<Box<dyn ExporterBuilder>, LogError> {
    let exporter: Box<dyn ExporterBuilder> = match protocol {
        Protocol::Tonic => Box::new(opentelemetry_otlp::new_exporter().tonic()),
        Protocol::Http => Box::new(opentelemetry_otlp::new_exporter().http()),
    };

    Ok(exporter)
}

pub async fn logs(protocol: &Protocol) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let exporter = init_logs(protocol)?;
    println!("init done..");
    let logger_provider = exporter.build()?;
    println!("build done..");
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    println!("bridge done..");
    log::set_boxed_logger(Box::new(otel_log_appender))?;
    println!("set logger done..");
    log::set_max_level(Level::Info.to_level_filter());
    println!("let's log..");
    info!(target: "my-target", "hello from {}. My price is {}.", "banana", 2.99);
    println!("log done..");
    let _ = logger_provider.shutdown();
    println!("shutdown done..");
    Ok(())
}

pub fn assert_logs_results(result: &str, expected: &str) {
    let left = read_logs_from_json(File::open(expected).unwrap());
    let right = read_logs_from_json(File::open(result).unwrap());

    LogsAsserter::new(left, right).assert();

    assert!(File::open(result).unwrap().metadata().unwrap().size() > 0)
}

#[test]
#[should_panic(expected = "assertion `left == right` failed: body does not match")]
pub fn test_assert_logs_eq_failure() {
    let left = read_logs_from_json(File::open("./expected/logs.json").unwrap());
    let right = read_logs_from_json(File::open("./expected/failed_logs.json").unwrap());
    LogsAsserter::new(right, left).assert();
}

#[test]
pub fn test_assert_logs_eq() {
    let logs = read_logs_from_json(File::open("./expected/logs.json").unwrap());
    LogsAsserter::new(logs.clone(), logs).assert();
}
