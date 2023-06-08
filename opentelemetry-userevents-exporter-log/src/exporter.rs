#![allow(unused_imports, unused_mut, unused_variables)]

use eventheader::{FieldFormat, Level, Opcode};
use eventheader_dynamic::{EventBuilder, EventSet};
use crate::exporter_traits::*;
use async_trait::async_trait;

use std::sync::Arc;
use std::fmt::Debug;

use std::{cell::RefCell, time::SystemTime};
use opentelemetry_api::logs::Severity;


thread_local! {static EBW: RefCell<EventBuilder> = RefCell::new(EventBuilder::new());}


pub(crate) struct UserEventsExporter<C: KeywordLevelProvider> {
    provider: Arc<eventheader_dynamic::Provider>,
    exporter_config: ExporterConfig<C>,
}

impl<C: KeywordLevelProvider> UserEventsExporter<C> {
    pub(crate) fn new(
        provider_name: &str,
        provider_group: ProviderGroup,
        exporter_config: ExporterConfig<C>,
    ) -> Self
    {
        let mut options = eventheader_dynamic::Provider::new_options();
        options = *options.group_name(provider_name);
        let mut provider: eventheader_dynamic::Provider = eventheader_dynamic::Provider::new(provider_name, &options);
        provider.register_set(eventheader::Level::Informational, exporter_config.get_log_event_keywords());
        provider.register_set(eventheader::Level::Error, exporter_config.get_log_event_keywords());
        provider.register_set(eventheader::Level::Verbose, exporter_config.get_log_event_keywords());

        provider.create_unregistered(true,eventheader::Level::Informational,exporter_config.get_log_event_keywords());
        provider.create_unregistered(true, eventheader::Level::Error, exporter_config.get_log_event_keywords());
        provider.create_unregistered(true, eventheader::Level::Verbose, exporter_config.get_log_event_keywords());

        UserEventsExporter { 
            provider: Arc::new(provider),
            exporter_config 
        }
    }

    fn enabled(&self, level: u8, keyword: u64) -> bool{
        let es = self.provider.find_set(level.into(), keyword);
        if es.is_some() {
            es.unwrap().enabled()
        } else {
            false
        }
    }

    fn export_log_data(&self,log_data: &opentelemetry_sdk::export::logs::LogData ) -> opentelemetry_sdk::export::logs::ExportResult {

        let level = match log_data.record.severity_number.unwrap() {
            Severity::Debug 
            | Severity::Debug2 
            | Severity::Debug3 
            | Severity::Debug4
            | Severity::Trace
            | Severity::Trace2
            | Severity::Trace3
            | Severity::Trace4 => eventheader::Level::Verbose,

            Severity::Info
            | Severity::Info2
            | Severity::Info3
            | Severity::Info4 => eventheader::Level::Informational,

            Severity::Error
            | Severity::Error2
            | Severity::Error3
            | Severity::Error4 => eventheader::Level::Error,

            Severity::Fatal
            | Severity::Fatal2
            | Severity::Fatal3
            | Severity::Fatal4 => eventheader::Level::CriticalError,

            Severity::Warn
            | Severity::Warn2
            | Severity::Warn3
            | Severity::Warn4 => eventheader::Level::Warning
        };
        let es = self.provider.find_set(level.as_int().into(), self.exporter_config.get_log_event_keywords());
        if (es.is_some()) && es.unwrap().enabled() {

            EBW.with(|eb| {

                let mut eb = eb.borrow_mut();
                


            });



            return Ok(());

        }

        Ok(())


    }
}

impl <C: KeywordLevelProvider> Debug for UserEventsExporter<C> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("user_events log exporter")
    }
}

#[async_trait]
impl <C: KeywordLevelProvider> opentelemetry_sdk::export::logs::LogExporter for UserEventsExporter<C> {
    async fn export(&mut self, batch: Vec<opentelemetry_sdk::export::logs::LogData>) -> opentelemetry_api::logs::LogResult<()> {
        for log_data in batch {
            let _ = self.export_log_data(&log_data);    
        }
        Ok(())
    } 
}