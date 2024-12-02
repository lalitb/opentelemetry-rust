/*
    Stress test results:
    OS: Ubuntu 22.04.4 LTS (5.15.153.1-microsoft-standard-WSL2)
    Hardware: Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz, 16vCPUs,
    RAM: 64.0 GB
    ~31 M/sec

    Hardware: AMD EPYC 7763 64-Core Processor - 2.44 GHz, 16vCPUs,
    ~44 M /sec
*/

use async_trait::async_trait;
use opentelemetry::InstrumentationScope;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::export::logs::{LogBatch, LogExporter};
use opentelemetry_sdk::logs::LogResult;
use opentelemetry_sdk::logs::{LogProcessor, LoggerProvider};
use std::sync::{Arc, Mutex};
use tracing::error;
use tracing_subscriber::prelude::*;

mod throughput;

#[derive(Clone, Debug, Default)]
pub struct SimpleExporter;

#[async_trait]
impl LogExporter for SimpleExporter {
    async fn export(&mut self, _batch: LogBatch<'_>) -> LogResult<()> {
        // Simulated export logic
        println!("Exporting logs...");
        Ok(())
    }

    fn shutdown(&mut self) {
        println!("Shutting down exporter...");
    }
}

#[derive(Debug)]
pub struct NoOpLogProcessor {
    exporter: Arc<Mutex<SimpleExporter>>,
}

impl NoOpLogProcessor {
    pub fn new(exporter: SimpleExporter) -> Self {
        Self {
            exporter: Arc::new(Mutex::new(exporter)),
        }
    }
}

impl LogProcessor for NoOpLogProcessor {
    fn emit(&self, record: &mut opentelemetry_sdk::logs::LogRecord, scope: &InstrumentationScope) {
        let log_tuple = &[(record as &opentelemetry_sdk::logs::LogRecord, scope)];
        let log_batch = LogBatch::new(log_tuple);

        // Access the exporter using the Mutex
        if let Ok(mut exporter) = self.exporter.lock() {
            let future = exporter.export(log_batch);
            match futures_executor::block_on(future) {
                Err(err) => {
                    eprintln!("Error: Export failed in NoOpLogProcessor. Error: {:?}", err);
                }
                _ => {}
            }
        } else {
            eprintln!("Error: Failed to acquire lock on exporter.");
        }
    }

    fn force_flush(&self) -> opentelemetry_sdk::logs::LogResult<()> {
        Ok(())
    }

    fn shutdown(&self) -> opentelemetry_sdk::logs::LogResult<()> {
        Ok(())
    }
}

fn main() {
    let exporter = SimpleExporter::default();
    let processor = NoOpLogProcessor::new(exporter);
    // LoggerProvider with a no-op processor.
    let provider: LoggerProvider = LoggerProvider::builder()
        .with_log_processor(processor)
        .build();

    // Use the OpenTelemetryTracingBridge to test the throughput of the appender-tracing.
    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();
    throughput::test_throughput(test_log);
}

fn test_log() {
    error!(
        name = "CheckoutFailed",
        book_id = "12345",
        book_title = "Rust Programming Adventures",
        message = "Unable to process checkout."
    );
}
