#![allow(unused_imports, unused_mut, unused_variables)]

use eventheader::{FieldFormat, Level, Opcode};
use eventheader_dynamic::{EventBuilder, EventSet};
use opentelemetry_sdk;
use crate::exporter_traits::*;

use std::sync::Arc;
use std::{cell::RefCell, time::SystemTime};


thread_local! {static EBW: RefCell<EventBuilder> = RefCell::new(EventBuilder::new());}


pub(crate) struct UserEventsExporter<C: KeywordLevelProvider> {
    provider: Arc<eventheader_dynamic::Provider>,
    exporter_config: ExporterConfig<C>,
}

impl<C: KeywordLevelProvider> UserEventsExporter<C> {
    pub(crate) fn new(
        provider: Arc<eventheader_dynamic::Provider>,
        exporter_config: ExporterConfig<C>,
        provider_name: &str,
        provider_group: ProviderGroup,
    ) -> Self
    {
        *provider.register_set(eventheader::Level::Informational, exporter_config.get_log_event_keywords());

        UserEventsExporter {
            provider,
            exporter_config,
        }


    }

}

