# Getting Started with OpenTelemetry Rust Logs in 5 Minutes - Exporting logs to user_events

## Prerequisites

* Rust and Cargo: [Install](https://doc.rust-lang.org/cargo/getting-started/installation.html) them if you haven't.
* A Linux system with [user_events](https://docs.kernel.org/trace/user_events.html) enabled: Ubuntu 23.10 (once available), or kernel 6.4 or later build with user_events support.
* Permissions: Make sure you're a Linux user with write permissions to the tracefs (preferably not root).
* Perf Tool: Install the [perf](https://perf.wiki.kernel.org/index.php/Main_Page) tool to capture the user_events.
* Decode-Perf Tool: Install the [decode-perf](https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-decode-cpp/tools) tool to decode the user_events.

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

3. Replace what's inside `src/main.rs` with the example file [basic.rs](./basic.rs). This new code sets up logging, make new tracepoints, and send an error log where it needs to go.
4. Run and Check: Go back to the terminal and type `cargo run`. If everything is okay, your logs will be sent to the proper place. To test this, you can use the `perf` and `decode-perf` tool like this:

```sh
$ perf record -e user_events:myprovider_L2K1Gmyprovider
<ctrl-c after running example code>
$ decode-perf ./perf.data
<log data is output here>
```

Note: The example code sets up a tracepoint called `myprovider_L2K1Gmyprovider`, and error log is exported to this tracepoint. Refer to the next section for more details.

## Configuring tracepoint names:

### Tracepoint Format:

ProviderName + '_' + 'L' + EventLevel + 'K' + EventKeyword + 'G' + ProviderGroup

### What you need to know:

* [ProviderName](https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-tracepoint#provider-names) and [EventKeyword](https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-tracepoint#tracepoint-names) can be changed using Exporter configuration.
* `ProviderGroup` is assigned the value of ProviderName.
* These settings decide the tracepoint names and where events go.
* OpenTelemetry SDK currently creates a tracepoint for each of the 5 levels (Critical, Error, Warning, Info, Verbose).
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
let processor = ReentrantLogProcessor::new("myprovider", None, exporter_config);
```
The `ProviderName` is configured as `myprovider` above.

3. Create LoggerProvider with above configurations.
```rust
let provider =  LoggerProvider::builder().with_log_processor(reenterant_processor).build();
```

### Example code:

The example code uses `myprovider` as the ProviderName and `1` as the EventKeyword. So, it makes these 5 tracepoints:

* myprovider_L5K1Gmyprovider
* myprovider_L4K1Gmyprovider
* myprovider_L3K1Gmyprovider
* myprovider_L2K1Gmyprovider
* myprovider_L1K1Gmyprovider

To get the events sent to these tracepoints, the listener should be set up to monitor one or more of them.
## Useful links

1. Rust Installation: https://doc.rust-lang.org/cargo/getting-started/installation.html
2. user_events: https://docs.kernel.org/trace/user_events.html
3. Trace point name format: https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-tracepoint#tracepoint-names
4. Perf tool: https://perf.wiki.kernel.org/index.php/Main_Page
5. decode-perf tool: https://github.com/microsoft/LinuxTracepoints/tree/main/libeventheader-decode-cpp/tools


