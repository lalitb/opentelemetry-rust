# Getting Started with OpenTelemetry Rust Logs in 5 Minutes - Exporting logs to user_events

## Prerequisites

* Rust and Cargo: Install them if you haven't.
* A Linux system: Make sure it supports user_events.
* Correct Permissions: Linux user(preferable non-root) with write permission to the tracefs.
* A listener/agent to listen for new events. Or else `perf` and `decode-perf` tools installed for validation.

## Steps
1. Create a new Rust application and run it.

```sh
$ cargo new otel-userevents-getting-started
$ cd otel-userevents-getting-started
$ cargo run
```

You'll see "Hello, world!".

2. Open the `Cargo.toml` file and paste these lines
```toml
[dependencies]
opentelemetry_sdk = "0.20.0"
opentelemetry-appender-tracing = "0.1.0"
opentelemetry-user-events-logs = "0.1.0"
tracing = { version = "0.1.37", default-features = false, features = ["std"] }
tracing-core = "0.1.31"
tracing-subscriber = { version = "0.3.0", default-features = false, features = ["registry", "std"] }

```

3. Replace what's inside `src/main.rs` with the example file `basic.rs`. This new code sets up logging, make new tracepoints, and send an error message where it needs to go.
4. Run and Check: Go back to the terminal and type `cargo run`. If everything is okay, your logs will be sent to the proper place. To test this, you can use the `perf` and `decode-perf` tool like this:

```sh
$ perf record -e user_events:test_L2K1Gtest
<ctrl-c after running sample code>
$ decode-perf ./perf.data
<log data is output here>
```

Note: The sample code sets up a tracepoint called `test_L2K1Gtest`, and error message are relayed to this tracepoint. Refer to next section for more details.

## Configuring tracepoint names:

### Tracepoint Format:
ProviderName + '_' + 'L' + EventLevel + 'K' + EventKeyword + 'G' + ProviderGroup

### What you need to know:

* `ProviderName` and `EventKeyword` can be changed using Exporter configuration.
* These settings decide the tracepoint names and where events go.
* OpenTelemetry SDK will automatically create a tracepoint for each of the 5 levels (Critical, Error, Warning, Info, Verbose).
* In future, the `EventKeyword` would be mapped to the unique events sent by the application.

### How to configure:

1. Create ExporterConfig with below settings:
```rust
    let exporter_config = ExporterConfig {
        default_keyword: 1,
        keywords_map: HashMap::new(),
    };
```
`EventKeyword` is configured with `default_keyword` value above.

2. Create LogProcessor instance:
```rust
let processor = ReentrantLogProcessor::new("test_provider", None, exporter_config);
```
The `ProviderName` is configured as `test_provider` above.

3. Create LoggerProvider with above configurations.
```rust
let provider =  LoggerProvider::builder().with_log_processor(reenterant_processor).build();
```

### Sample code:

The sample code uses `test` as the ProviderName and `1 as the EventKeyword. So, it makes these 5 tracepoints:

* test_L5K1Gtest
* test_L4K1Gtest
* test_L3K1Gtest
* test_L2K1Gtest
* test_L1K1Gtest

To get the events sent to these tracepoints, the listener should be set up to monitor one or more of them.


Useful Links:

1. Rust Installation: https://doc.rust-lang.org/cargo/getting-started/installation.html
2. user_events: https://docs.kernel.org/trace/user_events.html
3. Trace point name format: https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-tracepoint#tracepoint-names
4. Perf tool: https://perf.wiki.kernel.org/index.php/Main_Page
5. decode-perf tool: https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-decode-cpp/tools


