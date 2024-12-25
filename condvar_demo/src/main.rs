use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let data = Arc::new((Mutex::new(false), Condvar::new()));
    let data_clone = Arc::clone(&data);

    // 生产者线程
    thread::spawn(move || {
        let (lock, cvar) = &*data_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("hahahah!!!");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("hahahah!!!");
        cvar.notify_one(); // 通知消费者
    });
    println!("whooohsohdiasuda");
    // 消费者线程
    let (lock, cvar) = &*data;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("Thread started!");
}
