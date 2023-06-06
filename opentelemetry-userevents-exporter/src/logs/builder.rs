use opentelemetry_api::global;
use opentelemetry_api::global::GlobalLoggerProvider;
use opentelemetry_api::logs::LoggerProvider;

use crate::ExporterBuilder;
use crate::ExporterConfig;
use crate::DefaultKeywordLevelProvider;
use crate::ExporterAsyncRuntime;
use crate::logs;

pub struct LogsExporterBuilder{
    pub(crate) parent: ExporterBuilder,
    pub(crate) log_config: Option<opentelemetry_sdk::logs::Config>,
}

impl LogsExporterBuilder {
    // Install the exporter as the
    // [global logger provider](https://docs.rs/opentelemetry_api/latest/opentelemetry_api/global/index.html).
    pub fn install_log_exporter(
        mut self,
    ) -> <GlobalLoggerProvider as opentelemetry_api::logs::LoggerProvider>::Logger {
        self.parent.validate_config();
        let provider_builder = match self.parent.runtime {
            None => {
                let provider_builder = match self.parent.exporter_config {
                    Some(exporter_config) => {
                        opentelemetry_sdk::logs::LoggerProvider::builder()
                        .with_simple_exporter(logs::Ex)

                    }
                }    
            }
        };



    }
}