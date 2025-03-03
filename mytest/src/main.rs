use opentelemetry::global;
use opentelemetry_sdk::trace::Tracer;
use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};

#[instrument]
async fn demo_function(num: u32) -> u32 {
    info!("Processing number: {}", num);
    if num % 2 == 0 {
        debug!("Even number detected");
    } else {
        warn!("Odd number might need special handling");
    }
    num * 2
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // 初始化 Jaeger 追踪器
    let tracer: Tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("rust-jaeger-demo")
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    // 设置全局 tracing 订阅器
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber)?;

    // 创建一个根 span
    let root_span = span!(Level::INFO, "main_operation");
    let _guard = root_span.enter();

    info!("Starting main operation");

    // 模拟业务逻辑
    let result1 = demo_function(42).await;
    let result2 = demo_function(13).await;

    // 错误示例
    if let Err(e) = maybe_error() {
        error!(error = %e, "Encountered an error");
    }

    info!(results = ?vec![result1, result2], "Operation completed");

    // 关闭追踪器
    global::shutdown_tracer_provider();
    Ok(())
}

// 一个可能返回错误的函数
#[instrument]
fn maybe_error() -> Result<(), String> {
    Err("Something went wrong".to_string())
}
