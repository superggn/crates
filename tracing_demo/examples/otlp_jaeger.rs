/// https://github.com/tokio-rs/tracing-opentelemetry/blob/v0.1.x/examples/basic.rs
/// 此例子来自 tracing-otel 的 example
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as stdout;
use tracing::{Level, debug, error, info, instrument, span, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Registry, registry};
/// otel/jaeger side
/// trace => need SpanExporter
/// protocol (tonic grpc) + port => SpanExporter
/// exporter => TracerProvider (tracer template)

/// tracing side
/// tracing crate => need Layer
/// otel tracer + joint crate => tracing layer
/// layer + Registry => Subscriber
#[tokio::main]
async fn main() {
    // otel side
    // otlp exporter (use jaeger for demo)
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        // .
        .build()
        .unwrap();
    // std exporter
    let std_exporter = stdout::SpanExporter::default();
    // TracerProvider
    let resource = Resource::builder_empty()
        .with_attributes([KeyValue::new("service.name", "my-awesome-service")])
        .build();
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(std_exporter)
        .with_simple_exporter(otlp_exporter)
        .with_resource(resource)
        .build();
    // Tracer
    let tracer = provider.tracer("readme_example");

    // bridge
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // tracing side
    // Registry + bridge layer => tracing subscriber
    // let subscriber = Registry::default().with(telemetry).init();
    let subscriber = registry().with(telemetry);
    // subscriber.init();
    // subscriber.try_init().unwrap();
    // println!("do i need init?");
    // subscriber.try_init();

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root_span = span!(tracing::Level::TRACE, "app_start_hahaha", work_units = 2);
        let _enter = root_span.enter();
        error!("wuhu!! This event will be logged in the root span.");
        debug!("outer space debug, kid");
        info!("お前はもう， 死んでいる");
        // my_function(10);
    });
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    println!("123");
    // let sub_span = span!(Level::DEBUG, "my_sub_span", param_temp = 10, jojo = "1st");
    // info!("这个日志在 span 内部");
    // let _enter_2 = sub_span.enter();
    // info!("哈哈哈哈");
    // drop(_enter_2);
    // drop(sub_span);
    my_function(10);
    // subscriber;
    provider.force_flush().unwrap();
    println!("0");
    provider.shutdown().unwrap();
    println!("1");
}

#[instrument] // 自动创建一个 span
fn my_function(a: u32) {
    let x = 42;
    debug!(x, "计算完成");
}
