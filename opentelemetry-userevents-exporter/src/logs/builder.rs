use opentelemetry_api::global;
use opentelemetry_api::global::GlobalLoggerProvider;
use opentelemetry_api::logs::LoggerProvider;

use crate::ExporterBuilder;
use crate::ExporterConfig;
use crate::DefaultKeywordLevelProvider;
use crate::ExporterAsyncRuntime;
use crate::UserEventsExporter;
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
        if !self.parent.emit_realtime_events {
        let provider_builder = match self.parent.runtime {
            None => {
                let provider_builder = match self.parent.exporter_config {
                    Some(exporter_config) => {
                        opentelemetry_sdk::logs::LoggerProvider::builder()
                            .with_simple_exporter(logs::Exporter::new(
                                &self.parent.provider_name,
                                self.parent.provider_group,
                                ExporterConfig {
                                    kwl: exporter_config,
                                },
                            ))
                    }
                    None => opentelemetry_sdk::logs::LoggerProvider::builder()
                        .with_simple_exporter(logs::Exporter::new(
                        &self.parent.provider_name,
                        self.parent.provider_group,
                        ExporterConfig{
                            kwl: DefaultKeywordLevelProvider,
                        },
                    )),
                };
                if let Some(config) = self.log_config.take() {
                    provider_builder.with_config(config)
                } else {
                    provider_builder
                }
            }
            #[cfg(any(
                feature = "rt-tokio",
                feature = "rt-tokio-current-thread",
                feature = "rt-async-std"
            ))]
            Some(runtime) => {
                // If multiple runtimes are enabled this won't compile due to mismatched arms in the match
                let runtime = match runtime {
                    #[cfg(any(feature = "rt-tokio"))]
                    ExporterAsyncRuntime::Tokio => opentelemetry_sdk::runtime::Tokio,
                    #[cfg(any(feature = "rt-tokio-current-thread"))]
                    EtwExporterAsyncRuntime::TokioCurrentThread => {
                        opentelemetry_sdk::runtime::TokioCurrentThread
                    }
                    #[cfg(any(feature = "rt-async-std"))]
                    EtwExporterAsyncRuntime::AsyncStd => opentelemetry_sdk::runtime::AsyncStd,
                };
                let provider_builder = match self.parent.exporter_config {
                    Some(exporter_config) => {
                        opentelemetry_sdk::logs::LoggerProvider::builder().with_batch_exporter(
                            logs::exporter::new(
                                &self.parent.provider_name,
                                self.parent.provider_group,
                                ExporterConfig {
                                    kwl: exporter_config,
                                },
                            ),
                            runtime,
                        )
                    }
                    None => opentelemetry_sdk::logs::LoggerProvider::builder()
                        .with_batch_exporter(
                            logs::Exporter::new(
                                &self.parent.provider_name,
                                self.parent.provider_group,
                                ExporterConfig{
                                    kwl: DefaultKeywordLevelProvider,   
                                },
                            ),
                            runtime,
                        ),
                };

                if let Some(config) = self.log_config.take() {
                    provider_builder.with_config(config)
                } else {
                    provider_config
                }
            }
            #[cfg(not(any(
                feature = "rt-tokio",
                feature = "rt-tokio-current-thread",
                feature = "rt-async-std"
            )))]
            Some(_) => todo!(), // Unreachable
        };

        let provider = provider_builder.build();
        let _ = global::set_logger_provider(provider);
    } else {
        let exporter = Box::new(logs::Exporter::new(
            &self.parent.provider_name,
            self.parent.provider_group,
            ExporterConfig{
                kwl: DefaultKeywordLevelProvider,   
            },
        ));
        let processor = logs::RealTimeLogProcessor::new(exporter);
        // Process RealTimeLogProcessor

    }

        global::logger_provider().logger(
            "opentelemetry-user_events",
        )   

    }
}

 //   }
//}