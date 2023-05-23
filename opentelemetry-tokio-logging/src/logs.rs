use std::marker;

use opentelemetry_sdk::logs::{Logger, LoggerProvider};
use opentelemetry_api::{
    logs as otel,
    logs::{
        LogRecord, LogResult, 
    },
    Context as OtelContext,
};
