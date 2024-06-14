#![cfg(unix)]

use integration_test_runner::trace_asserter::{read_spans_from_json, TraceAsserter};
use integration_test_runner::Protocol;
use opentelemetry::global;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::trace::TraceError;
use opentelemetry::{
    trace::{TraceContextExt, Tracer},
    Key, KeyValue,
};
use opentelemetry_proto::tonic::trace::v1::TracesData;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;

trait ExporterBuilder: Send + Sync {
    fn build(self: Box<Self>) -> Result<sdktrace::TracerProvider, TraceError>;
}

impl ExporterBuilder for opentelemetry_otlp::TonicExporterBuilder {
    fn build(self: Box<Self>) -> Result<sdktrace::TracerProvider, TraceError> {
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    "basic-otlp-tracing-example",
                )])),
            )
            .with_exporter(*self)
            .install_batch(runtime::Tokio)
    }
}

impl ExporterBuilder for opentelemetry_otlp::HttpExporterBuilder {
    fn build(self: Box<Self>) -> Result<sdktrace::TracerProvider, TraceError> {
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(*self)
            .with_trace_config(
                sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    "basic-otlp-tracing-example",
                )])),
            )
            .install_batch(runtime::Tokio)
    }
}

fn init_traces(protocol: &Protocol) -> Result<Box<dyn ExporterBuilder>, TraceError> {
    let exporter: Box<dyn ExporterBuilder> = match protocol {
        Protocol::Tonic => Box::new(opentelemetry_otlp::new_exporter().tonic()),
        Protocol::Http => Box::new(opentelemetry_otlp::new_exporter().http()),
    };

    Ok(exporter)
}

const LEMONS_KEY: Key = Key::from_static_str("lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

pub async fn traces(protocol: &Protocol) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let exporter = init_traces(protocol).expect("Failed to initialize tracer provider.");
    let tracer_provider = exporter.build().expect("Failed to build tracer provider.");
    global::set_tracer_provider(tracer_provider.clone());

    let tracer = global::tracer("ex.com/basic");

    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attribute(KeyValue::new(ANOTHER_KEY, "yes"));

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new(LEMONS_KEY, "five"));

            span.add_event("Sub span event", vec![]);
        });
    });

    shutdown_tracer_provider();

    Ok(())
}

pub fn assert_traces_results(result: &str, expected: &str) {
    let left = read_spans_from_json(File::open(expected).unwrap());
    let right = read_spans_from_json(File::open(result).unwrap());

    TraceAsserter::new(left, right).assert();

    // we cannot read result json file because the timestamp was represents as string instead of u64.
    // need to fix it on json file exporter

    assert!(File::open(result).unwrap().metadata().unwrap().size() > 0)
}

#[test]
#[should_panic(expected = "left: \"Sub operation...\"")] // we swap the parent spans with child spans in failed_traces.json
pub fn test_assert_span_eq_failure() {
    let left = read_spans_from_json(File::open("./expected/traces.json").unwrap());
    let right = read_spans_from_json(File::open("./expected/failed_traces.json").unwrap());

    TraceAsserter::new(right, left).assert();
}

#[test]
pub fn test_assert_span_eq() {
    let spans = read_spans_from_json(File::open("./expected/traces.json").unwrap());

    TraceAsserter::new(spans.clone(), spans).assert();
}

#[test]
pub fn test_serde() {
    let spans = read_spans_from_json(
        File::open("./expected/traces.json").expect("Failed to read traces.json"),
    );
    let json = serde_json::to_string_pretty(&TracesData {
        resource_spans: spans,
    })
    .expect("Failed to serialize spans to json");

    // Write to file.
    let mut file = File::create("./expected/serialized_traces.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    let left = read_spans_from_json(
        File::open("./expected/traces.json").expect("Failed to read traces.json"),
    );
    let right = read_spans_from_json(
        File::open("./expected/serialized_traces.json")
            .expect("Failed to read serialized_traces.json"),
    );

    TraceAsserter::new(left, right).assert();
}
