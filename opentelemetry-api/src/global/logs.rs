use std::{
    borrow::Cow,
    fmt, mem,
    sync::{Arc, RwLock}, ops::Deref,
};

use once_cell::sync::Lazy;

use crate::{
    logs::{AnyValue, Logger, LogRecord, LoggerProvider, NoopLoggerProvider, Severity},
    InstrumentationLibrary,
};
use std::time::SystemTime;
use crate::trace::SpanContext;
use crate::Key;

// Allows a specific [`create::log::LogRecord`] to be used generically by [`BoxedLogRecord`]
// instances by mirroring the interface, and boxing the returned types.
pub trait ObjectSafeLogRecord {
    /// Records a timestamp for the log entry.
    ///
    /// # Arguments
    /// * `timestamp`: The timestamp to record.
    fn with_timestamp(&mut self, timestamp: SystemTime);

    /// Records the timestamp for when the log entry was observed by OpenTelemetry.
    ///
    /// # Arguments
    /// * `timestamp`: The timestamp to record.
    fn with_observed_timestamp(&mut self, timestamp: SystemTime);

    /// Assigns the log entry's span context.
    ///
    /// # Arguments
    /// * `span_context`: Reference to the span context to assign.
    fn with_span_context(&mut self, span_context: &SpanContext);

    /// Records the original severity text from the source.
    ///
    /// # Arguments
    /// * `severity_text`: The severity text to record.
    fn with_severity_text(&mut self, severity_text: Cow<'static, str>);

    /// Records the corresponding severity value, normalized.
    ///
    /// # Arguments
    /// * `severity_number`: The severity number to record.
    fn with_severity_number(&mut self, severity_number: Severity);

    /// Records the body of the log entry.
    ///
    /// # Arguments
    /// * `body`: The body to record.
    fn with_body(&mut self, body: AnyValue);

    /// Records additional attributes associated with the log entry.
    ///
    /// # Arguments
    /// * `attributes`: The attributes to record.
    fn with_attributes(&mut self, attributes: Vec<(Key, AnyValue)>);

    /// Records a single attribute associated with the log entry.
    ///
    /// # Arguments
    /// * `key`: The key of the attribute.
    /// * `value`: The value of the attribute.
    fn with_attribute(&mut self, key: Key, value: AnyValue);
}

impl<L : LogRecord> ObjectSafeLogRecord for L
{
    fn with_timestamp(&mut self, timestamp: SystemTime) {
        self.with_timestamp(timestamp)
    }

    fn with_observed_timestamp(&mut self, timestamp: SystemTime) {
        self.with_observed_timestamp(timestamp)
    }

    fn with_span_context(&mut self, span_context: &SpanContext) {
        self.with_span_context(span_context)
    }

    fn with_severity_text(&mut self, severity_text: Cow<'static, str>) {
        self.with_severity_text(severity_text)
    }

    fn with_severity_number(&mut self, severity_number: Severity) {
        self.with_severity_number(severity_number)
    }

    fn with_body(&mut self, body: AnyValue) {
        self.with_body(body)
    }

    fn with_attributes(&mut self, attributes: Vec<(Key, AnyValue)>) {
        self.with_attributes(attributes)
    }

    fn with_attribute(&mut self, key: Key, value: AnyValue) {
        self.with_attribute(key, value)
    }
}

/// Wraps the [`BoxedLogger`]'s [`LogRecord`] so it can be used generically by
/// applications without knowing the underlying type.
/// [`LogRecord`]: crate::logs::LogRecord
pub struct BoxedLogRecord(Box<dyn ObjectSafeLogRecord + Send + Sync + 'static>);

impl  BoxedLogRecord {
    pub(crate) fn new<R>(record: R) -> Self 
    where R: ObjectSafeLogRecord + Send + Sync + 'static
    {
        BoxedLogRecord(Box::new(record))
    }
}

impl fmt::Debug for BoxedLogRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BoxedLogRecord")
    }
}

impl LogRecord for BoxedLogRecord{
    fn with_timestamp(&mut self, timestamp: SystemTime) {
        self.0.with_timestamp(timestamp)
    }

    fn with_observed_timestamp(&mut self, timestamp: SystemTime) {
        self.0.with_observed_timestamp(timestamp)
    }

    fn with_span_context(&mut self, span_context: &SpanContext) {
        self.0.with_span_context(span_context)
    }

    fn with_severity_text(&mut self, severity_text: Cow<'static, str>) {
        self.0.with_severity_text(severity_text)
    }

    fn with_severity_number(&mut self, severity_number: Severity) {
        self.0.with_severity_number(severity_number)
    }

    fn with_body(&mut self, body: AnyValue) {
        self.0.with_body(body)
    }

    fn with_attributes(&mut self, attributes: Vec<(Key, AnyValue)>) {
        self.0.with_attributes(attributes)
    }

    fn with_attribute(&mut self, key: Key, value: AnyValue) {
        self.0.with_attribute(key, value)
    }
}


pub struct BoxedLogger(Box<dyn ObjectSafeLogger + Send + Sync>);


impl BoxedLogger {
    /// Create a `BoxedTracer` from an object-safe tracer.
    pub fn new(logger: Box<dyn ObjectSafeLogger + Send + Sync>) -> Self {
        BoxedLogger(logger)
    }
}

impl fmt::Debug for BoxedLogger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BoxedLogger")
    }
}

impl Logger for BoxedLogger {
    type LogRecord = BoxedLogRecord;

    fn create_log_record(&self) -> Self::LogRecord {
        BoxedLogRecord(self.0.create_log_record())
    }

    fn emit(&self, record: Self::LogRecord) {
        self.0.emit(record.0)
    }

    #[cfg(feature = "logs_level_enabled")]
    /// Check if the given log level is enabled.
    fn event_enabled(&self, level: Severity, target: &str) -> bool {
        self.0.event_enabled(level, target)
    }
}

pub trait ObjectSafeLogger {

    ///  Create a Log Record object
    fn create_log_record(&self) -> Box<dyn ObjectSafeLogRecord + Send + Sync>;

  /// Emit a [`LogRecord`]. If there is active current thread's [`Context`],
    ///  the logger will set the record's [`TraceContext`] to the active trace context,
    ///
    /// [`Context`]: crate::Context
    /// [`TraceContext`]: crate::logs::TraceContext
    fn emit(&self, record: Box<dyn ObjectSafeLogRecord + Send + Sync>);

    #[cfg(feature = "logs_level_enabled")]
    /// Check if the given log level is enabled.
    fn event_enabled(&self, level: Severity, target: &str) -> bool; 
}

impl<R, L> ObjectSafeLogger for L
where
   R: LogRecord + Send + Sync + 'static, 
   L: Logger<LogRecord = R>

{

    fn create_log_record(&self) -> Box<dyn ObjectSafeLogRecord + Send + Sync> {
        Box::new(self.create_log_record())
    }

    fn emit(&self, record: Box<dyn ObjectSafeLogRecord + Send + Sync>) {
        self.emit(record.deref());
    }

    #[cfg(feature = "logs_level_enabled")]
    /// Check if the given log level is enabled.
    fn event_enabled(&self, level: Severity, target: &str) -> bool {
        self.event_enabled(level, target)
    }
}

/// Wraps the [`GlobalLoggerProvider`]'s [`Logger`] so it can be used generically by
/// applications without knowing the underlying type.
///
/// [`Logger`]: crate::log::Logger
/// [`GlobalLoggerProvider`]: crate::global::GlobalLoggerProvider

///
/// [`Span`]: crate::trace::Span
/// Allows a specific [`LoggerProvider`] to be used generically, by mirroring
/// the interface, and boxing the returned types.
///
/// [`LoggerProvider`]: crate::logs::LoggerProvider.
pub trait ObjectSafeLoggerProvider {
    /// Creates a versioned named [`Logger`] instance that is a trait object
    /// through the underlying [`LoggerProvider`].
    ///
    /// [`Logger`]: crate::logs::Logger
    /// [`LoggerProvider`]: crate::logs::LoggerProvider
    fn boxed_logger(
        &self,
        library: Arc<InstrumentationLibrary>,
    ) -> Box<dyn ObjectSafeLogger + Send + Sync>;
}

impl<L, P> ObjectSafeLoggerProvider for P
where
    L: Logger + Send + Sync + 'static,
    P: LoggerProvider<Logger = L>,
{
    fn boxed_logger(
        &self,
        library: Arc<InstrumentationLibrary>,
    ) -> Box<dyn ObjectSafeLogger + Send + Sync> {
        Box::new(self.library_logger(library))
    }
}

#[derive(Clone)]
/// Represents the globally configured [`LoggerProvider`] instance.
pub struct GlobalLoggerProvider {
    provider: Arc<dyn ObjectSafeLoggerProvider + Send + Sync>,
}

impl fmt::Debug for GlobalLoggerProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GlobalLoggerProvider")
    }
}

impl GlobalLoggerProvider {
    fn new<
        L: Logger + Send + Sync + 'static,
        P: LoggerProvider<Logger = L> + Send + Sync + 'static,
    >(
        provider: P,
    ) -> Self {
        GlobalLoggerProvider {
            provider: Arc::new(provider),
        }
    }
}

impl LoggerProvider for GlobalLoggerProvider {
    type Logger = BoxedLogger;

    fn library_logger(&self, library: Arc<InstrumentationLibrary>) -> Self::Logger {
        BoxedLogger(self.provider.boxed_logger(library))
    }
}

static GLOBAL_LOGGER_PROVIDER: Lazy<RwLock<GlobalLoggerProvider>> =
    Lazy::new(|| RwLock::new(GlobalLoggerProvider::new(NoopLoggerProvider::new())));

/// Returns an instance of the currently configured global [`LoggerProvider`]
/// through [`GlobalLoggerProvider`].
///
/// [`LoggerProvider`]: crate::logs::LoggerProvider
pub fn logger_provider() -> GlobalLoggerProvider {
    GLOBAL_LOGGER_PROVIDER
        .read()
        .expect("GLOBAL_LOGGER_PROVIDER RwLock poisoned")
        .clone()
}

/// Creates a named instance of [`Logger`] via the configured
/// [`GlobalLoggerProvider`].
///
/// If `name` is an empty string, the provider will use a default name.
///
/// [`Logger`]: crate::logs::Logger
pub fn logger(name: Cow<'static, str>) -> BoxedLogger {
    logger_provider().logger(name)
}

/// Sets the given [`LoggerProvider`] instance as the current global provider,
/// returning the [`LoggerProvider`] instance that was previously set as global
/// provider.
pub fn set_logger_provider<L, P>(new_provider: P) -> GlobalLoggerProvider
where
    L: Logger + Send + Sync + 'static,
    P: LoggerProvider<Logger = L> + Send + Sync + 'static,
{
    let mut provider = GLOBAL_LOGGER_PROVIDER
        .write()
        .expect("GLOBAL_LOGGER_PROVIDER RwLock poisoned");
    mem::replace(&mut *provider, GlobalLoggerProvider::new(new_provider))
}

/// Shut down the current global [`LoggerProvider`].
pub fn shutdown_logger_provider() {
    let _ = set_logger_provider(NoopLoggerProvider::new());
}
