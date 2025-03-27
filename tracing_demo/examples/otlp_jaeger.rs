/// https://github.com/tokio-rs/tracing-opentelemetry/blob/v0.1.x/examples/basic.rs
/// 此例子来自 tracing-otel 的 example
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout as stdout;
use tracing::{Level, debug, error, info, instrument, span, warn};
use tracing_subscriber::{layer::SubscriberExt, registry};
/// otel/jaeger side
/// trace => need SpanExporter
/// protocol (tonic grpc) + port => SpanExporter
/// exporter => TracerProvider (tracer template)

/// tracing side
/// tracing crate => need Layer
/// otel tracer + joint crate => tracing layer
/// layer + Registry => Subscriber
///
/// 这里的 tokio::main 应该是 with_tonic 要求的环境，
/// 如果直接用 http + grpc 的话应该就用不上这个 tokio 了
#[tokio::main]
async fn main() {
    // otel side
    // otlp exporter (use jaeger for demo)
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()
        .unwrap();
    // std exporter
    let std_exporter = stdout::SpanExporter::default();
    // TracerProvider
    let resource = Resource::builder_empty()
        .with_attributes([KeyValue::new("service.name", "my-awesome-service")])
        .build();
    let provider = SdkTracerProvider::builder()
        // simple exporter => 会出现 "命名代码里打了很多 event, 但只传了1个" 这种情况
        //                 => 这时 jaeger 里就只能看到一条记录， 非常捉急
        //                 => 正常代码里应该用 BatchSpanProcessor 来着
        // .with_simple_exporter(std_exporter)
        // .with_simple_exporter(otlp_exporter)
        .with_resource(resource)
        .with_span_processor(
            opentelemetry_sdk::trace::BatchSpanProcessor::builder(std_exporter).build(),
        )
        .with_span_processor(
            opentelemetry_sdk::trace::BatchSpanProcessor::builder(otlp_exporter).build(),
        )
        .build();
    // Tracer
    let tracer = provider.tracer("readme_example");

    // bridge
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // tracing side
    // Registry + bridge layer => tracing subscriber
    // let subscriber = Registry::default().with(telemetry).init();
    let subscriber = registry().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        info!("お前はもう， 死んでいる");
        let root_span = span!(Level::TRACE, "app_start_hahaha", work_units = 2);
        info!("お前はもう， 死んでいる");
        let _enter = root_span.enter();
        let sub_span = span!(Level::TRACE, "哦豁， 啊哈！");
        info!("お前はもう， 死んでいる");
        let _enter_sub = sub_span.enter();
        error!("wuhu!! This event will be logged in the root span.");
        debug!("outer space debug, kid");
        info!("お前はもう， 死んでいる");
        my_function_outer(10);
    });
}

#[instrument] // 自动创建一个 span
fn my_function_outer(a: u32) {
    let x = 42;
    debug!(x, "计算完成");
    my_function_inner(2 * a);
}

#[instrument] // 自动创建一个 span
fn my_function_inner(a: u32) {
    let x = 9527;
    debug!(x, "计算完成");
}
