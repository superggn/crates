use tracing::{Level, debug, info, instrument, span, warn};

fn main() {
    // tracing_subscriber::fmt::init(); // 初始化日志
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        .init();
    debug!("test debug 0");
    let my_span = span!(Level::DEBUG, "my_span", param = 42);
    let _enter = my_span.enter(); // 进入 span
    // span 接收任意关键字传参， 这玩意儿可是一个 macro 啊！ 随便写参数
    let sub_span = span!(Level::DEBUG, "my_sub_span", param_temp = 10, jojo = "1st");
    info!("这个日志在 span 内部");
    let _enter_2 = sub_span.enter();
    info!("哈哈哈哈");
    debug!("test debug");
    warn!("now is warning!");
    // span 怎么 exit?
    drop(_enter_2);
    info!("now should be in my_span");
    drop(_enter);
    my_function(10);
}

#[instrument] // 自动创建一个 span
fn my_function(a: u32) {
    let x = 42;
    debug!(x, "计算完成");
}
