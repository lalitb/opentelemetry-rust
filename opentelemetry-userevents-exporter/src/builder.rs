use std::borrow::Cow;
use crate::logs;

use crate::exporter_traits::*;

type ProviderGroup = Option<Cow<'static, str>>;

/// Create a new exporter builder by calling [`new_exporter`].
pub struct ExporterBuilder {
    pub(crate) provider_name: String,
    pub(crate) provider_group: ProviderGroup,
    pub(crate) runtime: Option<ExporterAsyncRuntime>,
    pub(crate) exporter_config: Option<Box<dyn KeywordLevelProvider>>,
}

/// Create an exporter builder. After configuring the builder,
/// call [`ExporterBuilder::install`] to set it as the
/// [global tracer provider](https://docs.rs/opentelemetry_api/latest/opentelemetry_api/global/index.html).
pub fn new_exporter(name: &str) -> ExporterBuilder {
    ExporterBuilder {
        provider_name: name.to_owned(),
        provider_group: None,
        runtime: None,
        exporter_config: None
    }
}

impl ExporterBuilder {
    pub fn logs(self) -> logs::LogsExporterBuilder {
        logs::LogsExporterBuilder {
            parent: self,
            log_config: None
        }
    }

    pub fn with_provider_group(mut self, name: &str) -> Self {
        self.provider_group = ProviderGroup::Some(Cow::Owned(name.to_owned()));
        self
    }

    #[cfg(any(
        feature = "rt-tokio",
        feature = "rt-tokio-current-thread",
        feature = "rt-async-std",
        doc
    ))]
    pub fn with_async_runtime(mut self, runtime: EtwExporterAsyncRuntime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub(crate) fn validate_config(&self) {
        #[cfg(any(
            feature = "rt-tokio",
            feature = "rt-tokio-current-thread",
            feature = "rt-async-std"
        ))]
        match self.runtime.as_ref() {
            None => (),
            Some(x) => match x {
                #[cfg(any(feature = "rt-tokio"))]
                #[cfg(any(feature = "rt-tokio"))]
                EtwExporterAsyncRuntime::Tokio => (),
                #[cfg(any(feature = "rt-tokio-current-thread"))]
                EtwExporterAsyncRuntime::TokioCurrentThread => (),
                #[cfg(any(feature = "rt-async-std"))]
                EtwExporterAsyncRuntime::AsyncStd => (),
            },
        }
        assert!(eventheader_dynamic::ProviderOptions::is_valid_option_value(&self.provider_name),
        "Provider names must be lower case ASCII or numeric digits");
    }
}