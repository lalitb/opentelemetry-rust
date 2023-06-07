use std::{sync::{Arc, Weak}, borrow::Cow};
use std::fmt::Debug;

use opentelemetry_api::{InstrumentationLibrary, logs::LogRecord, logs::LogResult};
use opentelemetry_sdk::{export::logs::LogExporter, export::logs::LogData, export::logs::ExportResult};

use crate::{EventExporter, KeywordLevelProvider, UserEventsExporter, ExporterConfig, ProviderGroup};

#[derive(Debug)]
pub struct RealTimeLogProcessor<C: KeywordLevelProvider, E: EventExporter + Send + Sync + Debug> {
    event_exporter: Arc<E>,
    _x: core::marker::PhantomData<C>,
}

//impl<E: EventExporter + Send + Sync + Debug > RealTimeLogProcessor<E>{
impl<C: KeywordLevelProvider + Send + Sync + Debug> RealTimeLogProcessor<C, UserEventsExporter<C>> {
    pub(crate) fn new(
        provider_name: &str,
        provider_group: ProviderGroup,
        exporter_config: ExporterConfig<C>,
    ) -> Self{
        RealTimeLogProcessor{
            event_exporter: Arc
            _x: core::marker::PhantomData,
        }
    }
}

impl<C: KeywordLevelProvider + Debug, E: EventExporter + Send + Sync + Debug> opentelemetry_sdk::logs::LogProcessor for RealTimeLogProcessor<C, E> {

    fn emit(&self, data: LogData) {
    }

    fn force_flush(&self) -> LogResult<()> {
        Ok(())
    }

    fn shutdown(&mut self) -> LogResult<()>{
        Ok(())
    }
}