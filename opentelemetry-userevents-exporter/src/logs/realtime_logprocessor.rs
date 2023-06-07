use std::{sync::{Arc, Weak}, borrow::Cow};
use std::fmt::Debug;

use opentelemetry_api::{InstrumentationLibrary, logs::LogRecord, logs::LogResult};
use opentelemetry_sdk::{export::logs::LogExporter, export::logs::LogData, export::logs::ExportResult};

use crate::EventExporter;

#[derive(Debug)]
pub struct RealTimeLogProcessor<E: EventExporter + Send + Sync + Debug> {
    event_exporter: Weak<E>,
}

impl<E: EventExporter + Send + Sync + Debug > RealTimeLogProcessor<E>{
    fn new(
        event_exporter: Weak<E>
    ) -> Self{
        RealTimeLogProcessor{
            event_exporter
        }
    }
}

impl<E: EventExporter + Send + Sync + Debug> opentelemetry_sdk::logs::LogProcessor for RealTimeLogProcessor<E> {

    fn emit(&self, data: LogData) {
    }

    fn force_flush(&self) -> LogResult<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> LogResult<()>{
        Ok(())
    }
}