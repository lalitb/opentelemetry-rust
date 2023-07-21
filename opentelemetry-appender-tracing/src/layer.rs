use std::vec;

use opentelemetry_api::logs::{AnyValue, LogRecord, Logger, LoggerProvider, Severity};

use tracing_subscriber::Layer;

const INSTRUMENTATION_LIBRARY_NAME: &str = "opentelemetry-appender-tracing";

/// Visitor to record the fields from the event record.
struct EventVisitor<'a> {
    log_record: &'a mut LogRecord,
}

impl<'a> EventVisitor<'a> {
    fn record(&mut self, name: &str, value: AnyValue) {
        if let Some(ref mut vec) = self.log_record.attributes {
            vec.push((name.to_owned().into(), value));
        } else {
            self.log_record.attributes = Some(vec![(name.to_owned().into(), value)]);
        }
    }
}

impl<'a> tracing::field::Visit for EventVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let formatted_value = format!("{:?}", value);
        self.record(field.name(), AnyValue::String(formatted_value.into()));
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        self.record(field.name(), AnyValue::String(value.to_owned().into()));
    }

    fn record_bool(&mut self, field: &tracing_core::Field, value: bool) {
        self.record(field.name(), AnyValue::Boolean(value));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.record(field.name(), AnyValue::Double(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.record(field.name(), AnyValue::Int(value));
    }
    //TBD - support remaining types from AnyValue enum.
}

pub struct OpenTelemetryTracingBridge<P, L>
where
    P: LoggerProvider<Logger = L> + Send + Sync,
    L: Logger + Send + Sync,
{
    logger: L,
    _phantom: std::marker::PhantomData<P>, // P is not used.
}

impl<P, L> OpenTelemetryTracingBridge<P, L>
where
    P: LoggerProvider<Logger = L> + Send + Sync,
    L: Logger + Send + Sync,
{
    pub fn new(provider: &P) -> Self {
        OpenTelemetryTracingBridge {
            logger: provider.logger(INSTRUMENTATION_LIBRARY_NAME),
            _phantom: Default::default(),
        }
    }
}

impl<S, P, L> Layer<S> for OpenTelemetryTracingBridge<P, L>
where
    S: tracing::Subscriber,
    P: LoggerProvider<Logger = L> + Send + Sync + 'static,
    L: Logger + Send + Sync + 'static,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let meta = event.metadata();
        let mut log_record: LogRecord = LogRecord::default();
        log_record.severity_number = Some(map_severity_to_otel_severity(meta.level().as_str()));
        log_record.severity_text = Some(meta.level().to_string().into());

        let mut visitor = EventVisitor {
            log_record: &mut log_record,
        };
        event.record(&mut visitor);
        self.logger.emit(log_record);
    }
}

fn map_severity_to_otel_severity(level: &str) -> Severity {
    match level {
        "INFO" => Severity::Info,
        "DEBUG" => Severity::Debug,
        "TRACE" => Severity::Trace,
        "WARN" => Severity::Warn,
        "ERROR" => Severity::Error,
        _ => Severity::Info, // won't reach here
    }
}
