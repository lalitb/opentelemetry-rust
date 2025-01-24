//! Trace exporters
use crate::Resource;
use futures_util::future::BoxFuture;
use opentelemetry::trace::{SpanContext, SpanId, SpanKind, Status, TraceError};
use opentelemetry::{InstrumentationScope, KeyValue};
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::SystemTime;

/// A batch of spans to be exported by a `SpanExporter`.
///
/// The `SpanBatch` struct holds a collection of spans along with their associated
/// instrumentation scopes. This structure is used to group spans together for efficient
/// export operations.
///
/// # Type Parameters
/// - `'a`: The lifetime of the references to the log records and instrumentation scopes.
///
#[derive(Debug)]
pub struct SpanBatch<'a> {
    data: SpanBatchData<'a>,
}

/// The `SpanBatchData` enum represents the data field of a `SpanBatch`.
/// It can either be:
/// - A shared reference to a slice of boxed tuples, where each tuple consists of an owned `SpanData` and an owned `InstrumentationScope`.
/// - Or it can be a shared reference to a slice of tuples, where each tuple consists of a reference to a `LogRecord` and a reference to an `InstrumentationScope`.
#[derive(Debug)]
enum SpanBatchData<'a> {
    SliceOfOwnedData(&'a [Box<(SpanData, InstrumentationScope)>]), // Used by BatchProcessor which clones the LogRecords for its own use.
    SliceOfBorrowedData(&'a [(&'a SpanData, &'a InstrumentationScope)]),
}

impl<'a> SpanBatch<'a> {
    /// Creates a new instance of `LogBatch`.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of tuples, where each tuple consists of a reference to a `LogRecord`
    ///   and a reference to an `InstrumentationScope`. These tuples represent the log records
    ///   and their associated instrumentation scopes to be exported.
    ///
    /// # Returns
    ///
    /// A `LogBatch` instance containing the provided log records and instrumentation scopes.
    ///
    /// Note - this is not a public function, and should not be used directly. This would be
    /// made private in the future.
    pub fn new(data: &'a [(&'a SpanData, &'a InstrumentationScope)]) -> SpanBatch<'a> {
        SpanBatch {
            data: SpanBatchData::SliceOfBorrowedData(data),
        }
    }

    pub(crate) fn new_with_owned_data(
        data: &'a [Box<(SpanData, InstrumentationScope)>],
    ) -> SpanBatch<'a> {
        SpanBatch {
            data: SpanBatchData::SliceOfOwnedData(data),
        }
    }
}

impl SpanBatch<'_> {
    /// Returns an iterator over the log records and instrumentation scopes in the batch.
    ///
    /// Each item yielded by the iterator is a tuple containing references to a `LogRecord`
    /// and an `InstrumentationScope`.
    ///
    /// # Returns
    ///
    /// An iterator that yields references to the `LogRecord` and `InstrumentationScope` in the batch.
    ///
    pub fn iter(&self) -> impl Iterator<Item = (&SpanData, &InstrumentationScope)> {
        LogBatchDataIter {
            data: &self.data,
            index: 0,
        }
    }
}

struct LogBatchDataIter<'a> {
    data: &'a SpanBatchData<'a>,
    index: usize,
}

impl<'a> Iterator for LogBatchDataIter<'a> {
    type Item = (&'a SpanData, &'a InstrumentationScope);

    fn next(&mut self) -> Option<Self::Item> {
        match self.data {
            SpanBatchData::SliceOfOwnedData(data) => {
                if self.index < data.len() {
                    let record = &*data[self.index];
                    self.index += 1;
                    Some((&record.0, &record.1))
                } else {
                    None
                }
            }
            SpanBatchData::SliceOfBorrowedData(data) => {
                if self.index < data.len() {
                    let record = &data[self.index];
                    self.index += 1;
                    Some((record.0, record.1))
                } else {
                    None
                }
            }
        }
    }
}

/// Describes the result of an export.
pub type ExportResult = Result<(), TraceError>;

/// `SpanExporter` defines the interface that protocol-specific exporters must
/// implement so that they can be plugged into OpenTelemetry SDK and support
/// sending of telemetry data.
///
/// The goal of the interface is to minimize burden of implementation for
/// protocol-dependent telemetry exporters. The protocol exporter is expected to
/// be primarily a simple telemetry data encoder and transmitter.
pub trait SpanExporter: Send + Sync + Debug {
    /// Exports a batch of readable spans. Protocol exporters that will
    /// implement this function are typically expected to serialize and transmit
    /// the data to the destination.
    ///
    /// This function will never be called concurrently for the same exporter
    /// instance. It can be called again only after the current call returns.
    ///
    /// This function must not block indefinitely, there must be a reasonable
    /// upper limit after which the call must time out with an error result.
    ///
    /// Any retry logic that is required by the exporter is the responsibility
    /// of the exporter.
    fn export(&self, batch: SpanBatch<'_>) -> BoxFuture<'static, ExportResult>;

    /// Shuts down the exporter. Called when SDK is shut down. This is an
    /// opportunity for exporter to do any cleanup required.
    ///
    /// This function should be called only once for each `SpanExporter`
    /// instance. After the call to `shutdown`, subsequent calls to `export` are
    /// not allowed and should return an error.
    ///
    /// This function should not block indefinitely (e.g. if it attempts to
    /// flush the data and the destination is unavailable). SDK authors
    /// can decide if they want to make the shutdown timeout
    /// configurable.
    fn shutdown(&mut self) {}

    /// This is a hint to ensure that the export of any Spans the exporter
    /// has received prior to the call to this function SHOULD be completed
    /// as soon as possible, preferably before returning from this method.
    ///
    /// This function SHOULD provide a way to let the caller know
    /// whether it succeeded, failed or timed out.
    ///
    /// This function SHOULD only be called in cases where it is absolutely necessary,
    /// such as when using some FaaS providers that may suspend the process after
    /// an invocation, but before the exporter exports the completed spans.
    ///
    /// This function SHOULD complete or abort within some timeout. This function can be
    /// implemented as a blocking API or an asynchronous API which notifies the caller via
    /// a callback or an event. OpenTelemetry client authors can decide if they want to
    /// make the flush timeout configurable.
    fn force_flush(&mut self) -> BoxFuture<'static, ExportResult> {
        Box::pin(async { Ok(()) })
    }

    /// Set the resource for the exporter.
    fn set_resource(&mut self, _resource: &Resource) {}
}

/// `SpanData` contains all the information collected by a `Span` and can be used
/// by exporters as a standard input.
#[derive(Clone, Debug, PartialEq)]
pub struct SpanData {
    /// Exportable `SpanContext`
    pub span_context: SpanContext,
    /// Span parent id
    pub parent_span_id: SpanId,
    /// Span kind
    pub span_kind: SpanKind,
    /// Span name
    pub name: Cow<'static, str>,
    /// Span start time
    pub start_time: SystemTime,
    /// Span end time
    pub end_time: SystemTime,
    /// Span attributes
    pub attributes: Vec<KeyValue>,
    /// The number of attributes that were above the configured limit, and thus
    /// dropped.
    pub dropped_attributes_count: u32,
    /// Span events
    pub events: crate::trace::SpanEvents,
    /// Span Links
    pub links: crate::trace::SpanLinks,
    /// Span status
    pub status: Status,
}
