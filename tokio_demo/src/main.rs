/// tokio + crossbeam multithreads demo
use crossbeam::channel;
use futures::future::join_all;
use std::thread;
use std::time::Duration;
// use tokio::runtime::Runtime;
use tokio::{task, time::sleep};

async fn async_task(id: usize) {
    println!("Task {} is running!", id);
    sleep(Duration::from_secs(1)).await;
    println!("Task {} is done!", id);
}

fn main() {
    let m = 36; // 总共 m 个任务
    let n = 5; // 每个子线程要处理 n 个任务
    let x = 4; // 线程池的大小
    let (tx, rx) = channel::unbounded();

    // 向 channel 发送 m 个任务
    for i in 0..m {
        tx.send(i).unwrap();
    }

    // 创建一个线程池，池子里有 x 个线程
    let mut handles = Vec::new();
    for _ in 0..x {
        let rx = rx.clone(); // 克隆接收通道给每个线程
                             // 每个线程初始化时，新建一个 tokio runtime
        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                // .start_paused(true)
                .build()
                .unwrap();

            // let rt = Runtime::new().unwrap();
            // 现在代码里会有 if tasks.is_empty() 就直接结束任务的逻辑
            // 我想让他一直走， 就要把这块儿删掉
            rt.block_on(async {
                loop {
                    let mut tasks = Vec::new();
                    // 尝试从 channel 中接收 n 个任务
                    for _ in 0..n {
                        match rx.try_recv() {
                            Ok(task_id) => {
                                // 将任务加入待执行的异步任务列表
                                tasks.push(task::spawn(async_task(task_id)));
                            }
                            Err(_) => break, // 如果没有更多任务，退出循环
                        }
                    }
                    if tasks.is_empty() {
                        println!("No more tasks, shutting down.");
                        break;
                    }
                    // 并行地执行所有任务，并等待他们完成
                    join_all(tasks).await;
                }
            });
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All tasks are completed.");
}
