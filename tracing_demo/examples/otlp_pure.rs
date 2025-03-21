use opentelemetry::global;
use opentelemetry::trace::Tracer;

/// 时间
// use opentelemetry::sdk::{KeyValue, Resource};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Initialize OTLP exporter using gRPC (Tonic)
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    // Create a tracer provider with the exporter
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build();

    // Get a tracer and create spans
    let tracer = global::tracer("my_tracer");
    tracer.start("asd");
    tracer.in_span("doing_work", |_cx| {
        // Your application logic here...
        info!("hey here's my info!");
    });
    println!("here!");

    Ok(())
}
