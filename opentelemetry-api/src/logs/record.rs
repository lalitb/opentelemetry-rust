use crate::{
    trace::SpanContext,
    Array, Key, OrderMap, StringValue, Value,
};
use std::{borrow::Cow, time::SystemTime};


/// LogRecord represents all data carried by a log record, and
/// is provided to `LogExporter`s as input.
pub trait LogRecord {
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

/// Value types for representing arbitrary values in a log record.
#[derive(Debug, Clone)]
pub enum AnyValue {
    /// An integer value
    Int(i64),
    /// A double value
    Double(f64),
    /// A string value
    String(StringValue),
    /// A boolean value
    Boolean(bool),
    /// A byte array
    Bytes(Vec<u8>),
    /// An array of `Any` values
    ListAny(Vec<AnyValue>),
    /// A map of string keys to `Any` values, arbitrarily nested.
    Map(OrderMap<Key, AnyValue>),
}

macro_rules! impl_trivial_from {
    ($t:ty, $variant:path) => {
        impl From<$t> for AnyValue {
            fn from(val: $t) -> AnyValue {
                $variant(val.into())
            }
        }
    };
}

impl_trivial_from!(i8, AnyValue::Int);
impl_trivial_from!(i16, AnyValue::Int);
impl_trivial_from!(i32, AnyValue::Int);
impl_trivial_from!(i64, AnyValue::Int);

impl_trivial_from!(u8, AnyValue::Int);
impl_trivial_from!(u16, AnyValue::Int);
impl_trivial_from!(u32, AnyValue::Int);

impl_trivial_from!(f64, AnyValue::Double);
impl_trivial_from!(f32, AnyValue::Double);

impl_trivial_from!(String, AnyValue::String);
impl_trivial_from!(Cow<'static, str>, AnyValue::String);
impl_trivial_from!(&'static str, AnyValue::String);
impl_trivial_from!(StringValue, AnyValue::String);

impl_trivial_from!(bool, AnyValue::Boolean);

impl<T: Into<AnyValue>> FromIterator<T> for AnyValue {
    /// Creates an [`AnyValue::ListAny`] value from a sequence of `Into<AnyValue>` values.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        AnyValue::ListAny(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<Key>, V: Into<AnyValue>> FromIterator<(K, V)> for AnyValue {
    /// Creates an [`AnyValue::Map`] value from a sequence of key-value pairs
    /// that can be converted into a `Key` and `AnyValue` respectively.
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        AnyValue::Map(OrderMap::from_iter(
            iter.into_iter().map(|(k, v)| (k.into(), v.into())),
        ))
    }
}

impl From<Value> for AnyValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => b.into(),
            Value::I64(i) => i.into(),
            Value::F64(f) => f.into(),
            Value::String(s) => s.into(),
            Value::Array(a) => match a {
                Array::Bool(b) => AnyValue::from_iter(b),
                Array::F64(f) => AnyValue::from_iter(f),
                Array::I64(i) => AnyValue::from_iter(i),
                Array::String(s) => AnyValue::from_iter(s),
            },
        }
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
