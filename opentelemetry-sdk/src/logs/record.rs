
use opentelemetry_api::logs::{LogRecord, Logger, LoggerProvider, Severity};
use std::vec;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
/// LogRecord represents all data carried by a log record, and
/// is provided to `LogExporter`s as input.
pub struct LogRecord {
    /// Record timestamp
    pub timestamp: Option<SystemTime>,

    /// Timestamp for when the record was observed by OpenTelemetry
    pub observed_timestamp: Option<SystemTime>,

    /// Trace context for logs associated with spans
    pub trace_context: Option<TraceContext>,

    /// The original severity string from the source
    pub severity_text: Option<Cow<'static, str>>,
    /// The corresponding severity value, normalized
    pub severity_number: Option<Severity>,

    /// Record body
    pub body: Option<AnyValue>,

    /// Additional attributes associated with this record
    pub attributes: Option<Vec<(Key, AnyValue)>>,
}

impl opentelemetry_api::trace::LogRecord for LogRecord {     
    fn with_timestamp(self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    fn with_observed_timestamp(self, timestamp: SystemTime) -> Self {
        self.observed_timestamp = Some(timestamp);
        self
    }

    fn with_span_context(self, span_context: &SpanContext) -> Self {
        self.trace_context =  Some(TraceContext {
            span_id: span_context.span_id(),
            trace_id: span_context.trace_id(),
            trace_flags: Some(span_context.trace_flags()),
        });
        self
    }

    fn with_context<T>(self, context: &T) -> Self
    where
        T: TraceContextExt,
    {
        if context.has_active_span() {
            self.with_span_context(context.span().span_context())
        } else {
            self
        }
    }

    fn with_severity_text(self, severity_text: impl Into<Cow<'static, str>>) -> Self {
        self.severity_text = Some(severity_text.into());
        self
    }

    fn with_severity_number(self, severity_number: Severity) -> Self {
        self.severity_number = Some(severity_number);
        self
    }

    fn with_body(self, body: AnyValue) -> Self {
        self.body = Some(body);
        self
    }

    fn with_attributes(self, attributes: Vec<(Key, AnyValue)>) -> Self {
        self.attributes = Some(attributes);
        self
    }

    fn with_attribute<K,V>(self, key: K, value: V) -> Self
    where
        K: Into<Key>,
        V: Into<AnyValue>,
    {
        let mut attributes = self.attributes.unwrap_or_default();
        attributes.push((key.into(), value.into()));
        self.attributes = Some(attributes);
        self
    }
}

/// TraceContext stores the trace data for logs that have an associated
/// span.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct TraceContext {
    /// Trace id
    pub trace_id: TraceId,
    /// Span Id
    pub span_id: SpanId,
    /// Trace flags
    pub trace_flags: Option<TraceFlags>,
}

impl From<&SpanContext> for TraceContext {
    fn from(span_context: &SpanContext) -> Self {
        TraceContext {
            trace_id: span_context.trace_id(),
            span_id: span_context.span_id(),
            trace_flags: Some(span_context.trace_flags()),
        }
    }
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





