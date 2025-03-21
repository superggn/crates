/// https://github.com/tokio-rs/tracing-opentelemetry/blob/v0.1.x/examples/basic.rs
/// 此例子来自 tracing-otel 的 example
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as stdout;
use std::thread::sleep;
use std::time::Duration;
use tracing::{error, span};
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        // .with_tonic()
        .with_http()
        .with_endpoint("http://localhost:8000")
        .with_protocol(Protocol::HttpBinary)
        // .with_protocol(Protocol::HttpJson)
        .build()
        .unwrap();
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let std_exporter = stdout::SpanExporter::default();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .with_simple_exporter(std_exporter)
        .build();

    let tracer = provider.tracer("readme_example");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });
    sleep(Duration::from_secs(2));
    provider.shutdown().unwrap();
    println!("shutdown done!")
}
