use opentelemetry_api::logs::LogResult;
use opentelemetry_sdk::export::logs::{LogExporter, LogData};
use opentelemetry_sdk::logs::LogProcessor;
use std::fmt::Debug;
use std::sync::Arc;
use crate::exporter_traits::*;
use crate::user_events::*;
use crate::builder::*;
use async_trait::*;


pub(crate) struct Exporter<E: EventExporter + Send + Sync> {
    ebw: E,
}

impl<C: KeywordLevelProvider> Exporter<UserEventsExporter<C>> {
    pub(crate) fn new(
        provider_name: &str,
        provider_group: ProviderGroup,
        exporter_config: ExporterConfig<C>,
    ) -> Self {
        let mut options = eventheader_dynamic::Provider::new_options();
        options = *options.group_name(provider_group.as_deref().unwrap()); // TBD - Error handling
        let mut provider = eventheader_dynamic::Provider::new(provider_name, &options);
        register_eventsets(&mut provider, &exporter_config);
        Exporter {
            ebw: UserEventsExporter::new(Arc::new(provider), exporter_config),
        }
    }
}

impl<E: EventExporter + Send + Sync> Debug for Exporter<E> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl<E: EventExporter + Send + Sync> LogExporter for Exporter<E> {
    async fn export(&mut self, batch: Vec<LogData>) -> LogResult<()> {
        for log_data in batch {
            let _ = self.ebw.log_log_data(&log_data);
        }
        Ok(())
    }
}

