use std::{
    borrow::Cow,
    fmt, mem,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use once_cell::sync::Lazy;

use crate::{
    Key,
    logs::{AnyValue, Logger, LogRecord, LoggerProvider, NoopLoggerProvider, Severity},
    trace::SpanContext,
    InstrumentationLibrary
};

/// Allows a specific [`crate::trace::LogRecord`] to be used generically by [`BoxedLogRecord`]
/// instances by mirroring the interface and boxing the return types.
pub trait ObjectSafeLogRecord {
    fn with_timestamp(self, timestamp: SystemTime);

    fn with_observed_timestamp(self, timestamp: SystemTime);

    fn with_span_context(self, span_context: &SpanContext);

    fn with_severity_text(self, severity_text: Cow<'static, str>);

    fn with_severity_number(self, severity_number: Severity);

    fn with_body(self, body: AnyValue);

    fn with_attributes(self, attributes: Vec<(Key, AnyValue)>);

    fn with_attribute(self, key: Key, value: AnyValue);
}

impl<L: LogRecord> ObjectSafeLogRecord for L
{
    fn with_timestamp(self, timestamp: SystemTime) {
        self.with_timestamp(timestamp)
    }

    fn with_observed_timestamp(self, timestamp: SystemTime) {
        self.with_observed_timestamp(timestamp)
    }

    fn with_span_context(self, span_context: &SpanContext) {
        self.with_span_context(span_context)
    }

    fn with_severity_text(self, severity_text: Cow<'static, str>)
     {
        self.with_severity_text(severity_text)
    }

    fn with_severity_number(self, severity_number: Severity) {
        self.with_severity_number(severity_number)
    }

    fn with_body(self, body: AnyValue) {
        self.with_body(body)
    }

    fn with_attributes(self, attributes: Vec<(Key, AnyValue)>) {
        self.with_attributes(attributes)
    }

    fn with_attribute(self, key: Key, value: AnyValue)
    {
        self.with_attribute(key, value)
    }
}

pub struct BoxedLogRecord(Box<dyn ObjectSafeLogRecord + Send + Sync + 'static>);

impl BoxedLogRecord {
    pub(crate) fn new<L>(record: L) -> Self 
    where
        L: ObjectSafeLogRecord + Send + Sync + 'static,
    {
        BoxedLogRecord(Box::new(record))
    }

    pub fn into_inner(self) -> Box<dyn ObjectSafeLogRecord + Send + Sync + 'static> {
        self.0
    }
}


impl fmt::Debug for BoxedLogRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BoxedLogRecord")
    }
}

impl LogRecord for BoxedLogRecord {

    fn with_timestamp(self, timestamp: SystemTime) {
        self.0.with_timestamp(timestamp)
    }

    fn with_observed_timestamp(self, timestamp: SystemTime) {
        self.0.with_observed_timestamp(timestamp)
    }

    fn with_span_context(self, span_context: &SpanContext) {
        self.0.with_span_context(span_context)
    }

    fn with_severity_text(self, severity_text: Cow<'static, str>) {
        self.0.with_severity_text(severity_text)
    }

    fn with_severity_number(self, severity_number: Severity) {
        self.0.with_severity_number(severity_number)
    }

    fn with_body(self, body: AnyValue) {
        self.0.with_body(body)
    }

    fn with_attributes(self, attributes: Vec<(Key, AnyValue)>) {
        self.0.with_attributes(attributes)
    }

    fn with_attribute(self, key: Key, value: AnyValue)

    {
        self.0.with_attribute(key, value)
    }

}

/// Wraps the [`GlobalLoggerProvider`]'s [`Logger`] so it can be used generically by
/// applications without knowing the underlying type.

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
    /// Global logger uses `BoxedLogRecord`s so that it can be a global singleton,
    /// which is not possible if it takes generic type parameters.
    type LogRecordType = BoxedLogRecord;

    fn create_log_record(&self) -> Self::LogRecordType {
        BoxedLogRecord(self.0.create_log_record_boxed())
    }

    fn emit(&self, record: Self::LogRecordType) {
        self.0.emit(record.into_inner())
    }
}

/// Allows a specific [`Logger`] to be used generically by [`BoxedLogger`]
/// instances by mirroring the interface and boxing the return types.
///
/// [`Logger`]: crate::logs::Logger
/// 

pub trait ObjectSafeLogger {

    /// Creates a new [`LogRecord`] that is a trait object through the underlying
    /// [`Logger`].
    ///
    /// [`LogRecord`]: crate::logs::LogRecord
    /// [`Logger`]: crate::logs::Logger
    fn create_log_record_boxed(&self) -> Box<dyn ObjectSafeLogRecord + Send + Sync>;

    /// Emits a [`LogRecord`] that is a trait object through the underlying
    /// 
    fn emit(&self, record: Box<dyn ObjectSafeLogRecord + Send + Sync>);

}

impl<R, L> ObjectSafeLogger for L
where
    R: LogRecord + Send + Sync + 'static,   
    L: Logger<LogRecordType = R>
{  

    fn create_log_record_boxed(&self) ->  Box<dyn ObjectSafeLogRecord + Send + Sync> {
        Box::new(self.create_log_record())
    }

    fn emit(&self, record: Box<dyn ObjectSafeLogRecord + Send + Sync>) {
        self.emit(*record)
    }

}


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
    ) -> Box<dyn ObjectSafeLogger + Send + Sync + 'static>;
}

impl<R, L, P> ObjectSafeLoggerProvider for P
where
    R: LogRecord + Send + Sync + 'static,
    L: Logger<LogRecordType = R> + Send + Sync + 'static,
    P: LoggerProvider<Logger = L>,
{
    fn boxed_logger(
        &self,
        library: Arc<InstrumentationLibrary>,
    ) -> Box<dyn ObjectSafeLogger + Send + Sync + 'static> {
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
        R: LogRecord + Send + Sync + 'static,
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
