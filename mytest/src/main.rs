use tokio::time::{sleep, Duration};

async fn long_running_task() {
    println!("phase 1");
    sleep(Duration::from_secs(1)).await;
    println!("phase 2");
    sleep(Duration::from_secs(1)).await;
    println!("phase 3");
    sleep(Duration::from_secs(1)).await;
    println!("phase 4");
    sleep(Duration::from_secs(1)).await;
    println!("completed");
}

#[tokio::main]
async fn main() {
    println!("开始");

    // 使用 tokio::spawn 来并发执行任务
    let handle = tokio::spawn(long_running_task());

    // 这里会等待 1 秒钟再执行
    sleep(Duration::from_secs(10)).await;
    println!("waiting 10 秒");

    // 等待长时间任务完成
    handle.await.unwrap();

    println!("finish");
}
