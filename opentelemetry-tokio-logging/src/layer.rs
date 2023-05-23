
use std::borrow::Borrow;

use opentelemetry_api::{
    logs::LoggerProvider, logs::Severity, logs::{LogRecord, Logger},
    OrderMap
};

use opentelemetry_sdk:: {
    logs::Logger as sdk_logger,
    logs::LoggerProvider as sdk_log_provider
};

use tracing_subscriber::Layer;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTRUMENTATION_LIBRARY_NAME: &str = "opemtelemetry-tokio-logging";

/// Visitor to record the message from the event record
struct EventVisitor<'a> {
    log_record: &'a mut LogRecord
}

impl<'a> tracing::field::Visit for EventVisitor<'a> {

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        if field.name() == "message" {
            self.log_record.body = Some(value.to_string().into());
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.log_record.body = Some(format!("{value:?}").into());
            println!("LALIT: record debug: {:?}", self.log_record.body);
        }
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        if let Some(ref mut map) = self.log_record.attributes {
            map.insert(field.name().into(), value.into());
        } else {
            let mut map = OrderMap::with_capacity(1);
            map.insert(field.name().into(), value.into());
            self.log_record.attributes = Some(map);
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if let Some(ref mut map) = self.log_record.attributes {
            map.insert(field.name().into(), value.into());
        } else {
            let mut map = OrderMap::with_capacity(1);
            map.insert(field.name().into(), value.into());
            self.log_record.attributes = Some(map);
        }

    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        //Not supported type
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if let Some(ref mut map) = self.log_record.attributes {
            map.insert(field.name().into(), value.into());
        } else {
            let mut map = OrderMap::with_capacity(1);
            map.insert(field.name().into(), value.into());
            self.log_record.attributes = Some(map);
        }
    }


}

impl<'a> EventVisitor<'a>{
    fn update_severity(&mut self, level: &str){
        self.log_record.severity_text = Some(level.to_string().into());

        //self.log_record_builder.with_severity_text(level);
        match level {
            "Info" => self.log_record.severity_number =  Some(Severity::Info),
            "Debug" => self.log_record.severity_number =  Some(Severity::Debug),
            "Trace" => self.log_record.severity_number =  Some(Severity::Trace),
            "Warn" => self.log_record.severity_number =  Some(Severity::Warn),
            "Error" =>self.log_record.severity_number =  Some(Severity::Error),
            _ => self.log_record.severity_number = Some(Severity::Info) // won't reach here
        };
    }
}

pub struct OpenTelemetryLogsLayer{
    logger: sdk_logger,
}

impl OpenTelemetryLogsLayer{
    pub fn new(log_provider: sdk_log_provider) -> Self{
        let logger = log_provider.versioned_logger(INSTRUMENTATION_LIBRARY_NAME, Some(CARGO_PKG_VERSION.into()), None, None, true);
        Self {
            logger
        }
    }
}

impl<S> Layer<S> for OpenTelemetryLogsLayer
where
    S: tracing::Subscriber
    {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>
        ) {
            println!("LALIT->Event received");
            let meta = event.metadata();
            let mut log_record: LogRecord = LogRecord::default();
            let mut visitor = EventVisitor{log_record: &mut log_record};
            visitor.update_severity( meta.level().as_str());
            event.record(&mut visitor);
            self.logger.emit(log_record);
        }
    }