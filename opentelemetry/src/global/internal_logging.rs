#![allow(unused_macros)]
///
/// **Note**: These macros (`otel_info!`, `otel_warn!`, `otel_debug!`, and `otel_error!`) are intended to be used
/// **internally within OpenTelemetry code** or for **custom exporters and processors**. They are not designed
/// for general application logging and should not be used for that purpose.
///

/// Macro for logging informational messages in OpenTelemetry.
///
/// # Fields:
/// - `name`: The operation or action being logged.
/// - Additional optional key-value pairs can be passed as attributes.
///
/// # Example:
/// ```rust
/// otel_info!("sdk_start", version = "1.0.0", schema_url = "http://example.com");
/// ```
#[macro_export]
macro_rules! otel_info {
    (name: $name:expr $(,)?) => {
        {
            tracing::info!( target: env!("CARGO_PKG_NAME"), name= $name,"");
        }
    };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => {
        {
            tracing::info!(target: env!("CARGO_PKG_NAME"), name= $name, $($key = $value),+, "");
        }
    };
}

/// Macro for logging warning messages in OpenTelemetry.
///
/// # Fields:
/// - `name`: The operation or action being logged.
/// - Additional optional key-value pairs can be passed as attributes.
///
/// # Example:
/// ```rust
/// otel_warn!("export_warning", error_code = 404, version = "1.0.0");
/// ```
#[macro_export]
macro_rules! otel_warn {
    (name: $name:expr $(,)?) => {
        {
            tracing::warn!(target: env!("CARGO_PKG_NAME"), name= $name, "");
        }
    };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => {
        {
            tracing::warn!(target: env!("CARGO_PKG_NAME"), name= $name, $($key = $value),+, "");
        }
    };
}

/// Macro for logging debug messages in OpenTelemetry.
///
/// # Fields:
/// - `name`: The operation or action being logged.
/// - Additional optional key-value pairs can be passed as attributes.
///
/// # Example:
/// ```rust
/// otel_debug!("debug_operation", debug_level = "high", version = "1.0.0");
/// ```
#[macro_export]
macro_rules! otel_debug {
    (name: $name:expr $(,)?) => {
        {
            tracing::debug!(target: env!("CARGO_PKG_NAME"), name= $name,"");
        }
    };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => {
        {
            tracing::debug!(target: env!("CARGO_PKG_NAME"), name= $name, $($key = $value),+, "");
        }
    };
}

/// Macro for logging error messages in OpenTelemetry.
///
/// # Fields:
/// - `name`: The operation or action being logged.
/// - Additional optional key-value pairs can be passed as attributes.
///
/// # Example:
/// ```rust
/// otel_error!("export_failure", error_code = 500, version = "1.0.0");
/// ```
#[macro_export]
macro_rules! otel_error {
    (name: $name:expr $(,)?) => {
        {
            tracing::error!(target: env!("CARGO_PKG_NAME"), name= $name, "");
        }
    };
    (name: $name:expr, $($key:ident = $value:expr),+ $(,)?) => {
        {
            tracing::error!(target: env!("CARGO_PKG_NAME"), name= $name, $($key = $value),+, "");
        }
    };
}
