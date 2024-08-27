use super::StringValue as LogStringValue;
use crate::Key;
use std::{borrow::Cow, collections::HashMap, time::SystemTime};

/// SDK implemented trait for managing log records
pub trait LogRecord {
    /// Sets the `event_name` of a record
    fn set_event_name(&mut self, name: &'static str);

    /// Sets the `target` of a record.
    /// Currently, both `opentelemetry-appender-tracing` and `opentelemetry-appender-log` create a single logger
    /// with a scope that doesn't accurately reflect the component emitting the logs.
    /// Exporters MAY use this field to override the `instrumentation_scope.name`.
    fn set_target<T>(&mut self, _target: T)
    where
        T: Into<Cow<'static, str>>;

    /// Sets the time when the event occurred measured by the origin clock, i.e. the time at the source.
    fn set_timestamp(&mut self, timestamp: SystemTime);

    /// Sets the observed event timestamp.
    fn set_observed_timestamp(&mut self, timestamp: SystemTime);

    /// Sets severity as text.
    fn set_severity_text(&mut self, text: &'static str);

    /// Sets severity as a numeric value.
    fn set_severity_number(&mut self, number: Severity);

    /// Sets the message body of the log.
    fn set_body(&mut self, body: AnyValue<'_>);

    /// Adds multiple attributes.
    fn add_attributes<'a, I, K, V>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Key>,
        V: Into<AnyValue<'a>>;

    /// Adds a single attribute.
    fn add_attribute<'a, K, V>(&mut self, key: K, value: V)
    where
        K: Into<Key>,
        V: Into<AnyValue<'a>>;
}

/// Value types for representing arbitrary values in a log record.
#[derive(Debug, Clone, PartialEq)]
pub enum AnyValue<'a> {
    /// An integer value
    Int(i64),
    /// A double value
    Double(f64),
    /// A string value
    String(LogStringValue<'a>),
    /// A boolean value
    Boolean(bool),
    /// A byte array
    Bytes(Box<Vec<u8>>),
    /// An array of `Any` values
    ListAny(Box<Vec<AnyValue<'a>>>),
    /// A map of string keys to `Any` values, arbitrarily nested.
    Map(Box<HashMap<Key, AnyValue<'a>>>),
}

macro_rules! impl_trivial_from {
    ($t:ty, $variant:path) => {
        impl<'a> From<$t> for AnyValue<'a> {
            fn from(val: $t) -> AnyValue<'a> {
                $variant(val.into())
            }
        }
    };
}

// Integer types
impl_trivial_from!(i8, AnyValue::Int);
impl_trivial_from!(i16, AnyValue::Int);
impl_trivial_from!(i32, AnyValue::Int);
impl_trivial_from!(i64, AnyValue::Int);
impl_trivial_from!(u8, AnyValue::Int);
impl_trivial_from!(u16, AnyValue::Int);
impl_trivial_from!(u32, AnyValue::Int);

// Floating-point types
impl_trivial_from!(f64, AnyValue::Double);
impl_trivial_from!(f32, AnyValue::Double);

// Boolean type
impl_trivial_from!(bool, AnyValue::Boolean);

// String-related types
impl<'a> From<String> for AnyValue<'a> {
    fn from(val: String) -> AnyValue<'a> {
        AnyValue::String(LogStringValue::from(val))
    }
}

impl<'a> From<&'a str> for AnyValue<'a> {
    fn from(val: &'a str) -> AnyValue<'a> {
        AnyValue::String(LogStringValue::from(val))
    }
}

impl<'a> From<Cow<'a, str>> for AnyValue<'a> {
    fn from(val: Cow<'a, str>) -> AnyValue<'a> {
        AnyValue::String(LogStringValue::from(val))
    }
}

impl<'a> From<LogStringValue<'a>> for AnyValue<'a> {
    fn from(val: LogStringValue<'a>) -> AnyValue<'a> {
        AnyValue::String(val)
    }
}

// Implementation for static string references explicitly using StringValue
impl<'a> AnyValue<'a> {
    pub fn from_static_str(s: &'static str) -> Self {
        AnyValue::String(LogStringValue::from_static(s))
    }
}

// Implementation for byte array
impl<'a> From<Vec<u8>> for AnyValue<'a> {
    fn from(val: Vec<u8>) -> AnyValue<'a> {
        AnyValue::Bytes(Box::new(val))
    }
}

// Implementing FromIterator for lists and maps
impl<'a, T: Into<AnyValue<'a>>> FromIterator<T> for AnyValue<'a> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        AnyValue::ListAny(Box::new(iter.into_iter().map(Into::into).collect()))
    }
}

impl<'a, K: Into<Key>, V: Into<AnyValue<'a>>> FromIterator<(K, V)> for AnyValue<'a> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        AnyValue::Map(Box::new(HashMap::from_iter(
            iter.into_iter().map(|(k, v)| (k.into(), v.into())),
        )))
    }
}

/// A normalized severity value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Severity {
    /// TRACE
    Trace = 1,
    /// TRACE2
    Trace2 = 2,
    /// TRACE3
    Trace3 = 3,
    /// TRACE4
    Trace4 = 4,
    /// DEBUG
    Debug = 5,
    /// DEBUG2
    Debug2 = 6,
    /// DEBUG3
    Debug3 = 7,
    /// DEBUG4
    Debug4 = 8,
    /// INFO
    Info = 9,
    /// INFO2
    Info2 = 10,
    /// INFO3
    Info3 = 11,
    /// INFO4
    Info4 = 12,
    /// WARN
    Warn = 13,
    /// WARN2
    Warn2 = 14,
    /// WARN3
    Warn3 = 15,
    /// WARN4
    Warn4 = 16,
    /// ERROR
    Error = 17,
    /// ERROR2
    Error2 = 18,
    /// ERROR3
    Error3 = 19,
    /// ERROR4
    Error4 = 20,
    /// FATAL
    Fatal = 21,
    /// FATAL2
    Fatal2 = 22,
    /// FATAL3
    Fatal3 = 23,
    /// FATAL4
    Fatal4 = 24,
}

impl Severity {
    /// Return the string representing the short name for the `Severity`
    /// value as specified by the OpenTelemetry logs data model.
    pub const fn name(&self) -> &'static str {
        match &self {
            Severity::Trace => "TRACE",
            Severity::Trace2 => "TRACE2",
            Severity::Trace3 => "TRACE3",
            Severity::Trace4 => "TRACE4",

            Severity::Debug => "DEBUG",
            Severity::Debug2 => "DEBUG2",
            Severity::Debug3 => "DEBUG3",
            Severity::Debug4 => "DEBUG4",

            Severity::Info => "INFO",
            Severity::Info2 => "INFO2",
            Severity::Info3 => "INFO3",
            Severity::Info4 => "INFO4",

            Severity::Warn => "WARN",
            Severity::Warn2 => "WARN2",
            Severity::Warn3 => "WARN3",
            Severity::Warn4 => "WARN4",

            Severity::Error => "ERROR",
            Severity::Error2 => "ERROR2",
            Severity::Error3 => "ERROR3",
            Severity::Error4 => "ERROR4",

            Severity::Fatal => "FATAL",
            Severity::Fatal2 => "FATAL2",
            Severity::Fatal3 => "FATAL3",
            Severity::Fatal4 => "FATAL4",
        }
    }
}
