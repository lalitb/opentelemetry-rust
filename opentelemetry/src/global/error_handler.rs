    use std::sync::PoisonError;
    use std::sync::RwLock;

    #[cfg(feature = "logs")]
    use crate::logs::LogError;
    #[cfg(feature = "metrics")]
    use crate::metrics::MetricsError;
    use crate::propagation::PropagationError;
    #[cfg(feature = "trace")]
    use crate::trace::TraceError;
    use once_cell::sync::Lazy;

    /// Log levels for different severity.
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub enum LogLevel {
        Debug,
        Info,
        Warn,
        Error,
    }

    /// Pillars for different parts of the telemetry system (e.g., traces, metrics, logs)
    #[derive(Debug, Clone, Copy)]
    pub enum Pillar {
        Trace,
        Metrics,
        Logs,
        Propagation,
        Other,
    }

    /// Components within each pillar (e.g., SpanExporter, MetricProcessor)
    #[derive(Debug, Clone, Copy)]
    pub enum Component {
        Exporter,
        Processor,
        SpanProvider,
        Context,
        Other,
    }

    /// Struct for internal SDK errors, including metadata and log level.
    #[derive(Debug)]
    pub struct TelemetryLog  {
        pub level: LogLevel,
        pub pillar: Pillar,
        pub component: Component,
        pub message: String,
    }

    /// Compile-time global log level (set at compile time)
    const COMPILE_TIME_LOG_LEVEL: LogLevel = LogLevel::Info;  // This can be set at compile time

    /// Runtime global log level for filtering logs dynamically
    static GLOBAL_LOG_LEVEL: Lazy<RwLock<LogLevel>> = Lazy::new(|| RwLock::new(LogLevel::Info));

    /// Global log handler (can be customized by users)
    static GLOBAL_LOG_HANDLER: Lazy<RwLock<Option<LogHandler>>> = Lazy::new(|| RwLock::new(None));

    /// Handler for logs, with LogLevel and formatted message
    struct LogHandler(Box<dyn Fn(LogLevel, String) + Send + Sync>);

    /// Set the global log level for filtering log messages
    pub fn set_log_level(level: LogLevel) {
        *GLOBAL_LOG_LEVEL.write().unwrap() = level;
    }


    /// Runtime check if a log level is enabled based on the current global log level
    pub fn is_runtime_log_level_enabled(level: LogLevel) -> bool {
        *GLOBAL_LOG_LEVEL.read().unwrap() <= level
    }

    #[macro_export]
    /// Compile-time check macro for log levels
    macro_rules! is_compile_time_log_level_enabled {
        ($level:expr) => {
            $level >= COMPILE_TIME_LOG_LEVEL
        };
    }

    /// Logging Macros with compile-time and runtime checks

    #[macro_export]
    macro_rules! otel_log_debug {
        ($message:expr, $pillar:expr, $component:expr) => {
            if is_compile_time_log_level_enabled!(LogLevel::Debug) && is_runtime_log_level_enabled(LogLevel::Debug) {
                handle_log(Error {
                    level: LogLevel::Debug,
                    pillar: $pillar,
                    component: $component,
                    message: $message.to_string(),
                });
            }
        };
    }

    #[macro_export]
    macro_rules! otel_log_info {
        ($message:expr, $pillar:expr, $component:expr) => {
            if is_compile_time_log_level_enabled!(LogLevel::Info) && is_runtime_log_level_enabled(LogLevel::Info) {
                handle_log(Error {
                    level: LogLevel::Info,
                    pillar: $pillar,
                    component: $component,
                    message: $message.to_string(),
                });
            }
        };
    }

    #[macro_export]
    macro_rules! otel_log_warn {
        ($message:expr, $pillar:expr, $component:expr) => {
            if is_compile_time_log_level_enabled!(LogLevel::Warn) && is_runtime_log_level_enabled(LogLevel::Warn) {
                handle_log(Error {
                    level: LogLevel::Warn,
                    pillar: $pillar,
                    component: $component,
                    message: $message.to_string(),
                });
            }
        };
    }

    #[macro_export]
    macro_rules! otel_log_error {
        ($message:expr, $pillar:expr, $component:expr) => {
            if is_compile_time_log_level_enabled!(LogLevel::Error) && is_runtime_log_level_enabled(LogLevel::Error) {
                handle_log(Error {
                    level: LogLevel::Error,
                    pillar: $pillar,
                    component: $component,
                    message: $message.to_string(),
                });
            }
        };
    }

    /// Generic log handler for different log levels
    pub fn handle_log(error: TelemetryLog) {
        match GLOBAL_LOG_HANDLER.read() {
            Ok(handler) if handler.is_some() => {
                (handler.as_ref().unwrap().0)(error.level, format!("{:?}", error));
            }
            _ => eprintln!("[{:?}][{:?}][{:?}] {}", error.level, error.pillar, error.component, error.message),
        }
    }

    /// Set a custom global log handler
    pub fn set_log_handler<F>(f: F) -> std::result::Result<(), TelemetryLog>
    where
        F: Fn(LogLevel, String) + Send + Sync + 'static,
    {
        GLOBAL_LOG_HANDLER
            .write()
            .map(|mut handler| *handler = Some(LogHandler(Box::new(f))))
            .map_err(|_| TelemetryLog  {
                level: LogLevel::Error,
                pillar: Pillar::Other,
                component: Component::Other,
                message: "Failed to set log handler".to_string(),
            })
    }



    /// Wrapper for error from both tracing and metrics part of open telemetry.
    #[derive(thiserror::Error, Debug)]
    #[non_exhaustive]
    pub enum Error {
        #[cfg(feature = "trace")]
        #[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
        #[error(transparent)]
        /// Failed to export traces.
        Trace(#[from] TraceError),
        #[cfg(feature = "metrics")]
        #[cfg_attr(docsrs, doc(cfg(feature = "metrics")))]
        #[error(transparent)]
        /// An issue raised by the metrics module.
        Metric(#[from] MetricsError),

        #[cfg(feature = "logs")]
        #[cfg_attr(docsrs, doc(cfg(feature = "logs")))]
        #[error(transparent)]
        /// Failed to export logs.
        Log(#[from] LogError),

        #[error(transparent)]
        /// Error happens when injecting and extracting information using propagators.
        Propagation(#[from] PropagationError),

        #[error("{0}")]
        /// Other types of failures not covered by the variants above.
        Other(String),
    }

    impl<T> From<PoisonError<T>> for Error {
        fn from(err: PoisonError<T>) -> Self {
            Error::Other(err.to_string())
        }
    }