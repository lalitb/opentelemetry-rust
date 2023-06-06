#![allow(unused_imports, unused_mut, unused_variables)]

use eventheader::{FieldFormat, Level, Opcode};
use eventheader_dynamic::{EventBuilder, EventSet};

use opentelemetry::{
    Array, Key, Value,
};

use opentelemetry_sdk::export::{logs::{ExportResult, LogData, self}};
use std::{cell::RefCell, sync::Arc, time::SystemTime};
use crate::exporter_traits::*;

thread_local! {static EBW: RefCell<EventBuilder> = RefCell::new(EventBuilder::new());}

#[allow(dead_code)]
pub(crate) fn register_eventsets(
    provider: &mut eventheader_dynamic::Provider,
    kwl: &impl KeywordLevelProvider,
){
    #[cfg(not(test))]
    {
        provider.register_set(eventheader::Level::Informational, kwl.get_log_event_level());
        provider.register_set(eventheader::Level::Error, kwl.get_log_event_level());
        provider.register_set(eventheader::Level::Verbose, kwl.get_log_event_level()); 
    }
    #[cfg(test)]
    {
        provider.create_unregistered(true, eventheader::Level::Informational, kwl.get_log_event_level());
        provider.create_unregistered(true, eventheader::Level::Error, kwl.get_log_event_level());
        provider.create_unregistered(true, eventheader::Level::Verbose, kwl.get_log_event_level());    
    }  
}

pub(crate) struct UserEventsExporter<C: KeywordLevelProvider> {
    provider: Arc<eventheader_dynamic::Provider>,
    exporter_config: ExporterConfig<C>,
}

impl<C: KeywordLevelProvider> UserEventsExporter<C> {
    #[allow(dead_code)]
    pub(crate) fn new(
        provider: Arc<eventheader_dynamic::Provider>,
        exporter_config: ExporterConfig<C>,
    ) -> Self {
        // Unfortunately we can't safely share a cached EventBuilder without adding undesirable locking
        UserEventsExporter {
            provider,
            exporter_config,
        }
    }

    fn log_log_data(&self, log_data: &LogData) -> logs::ExportResult {
        Ok(())
    }
}




