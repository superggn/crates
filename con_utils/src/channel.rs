use anyhow::anyhow;
use anyhow::Result;

use std::{
    collections::VecDeque,
    sync::{atomic::AtomicUsize, atomic::Ordering, Arc, Condvar, Mutex, MutexGuard},
};

/// 发送者
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

/// 接收者
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

/// 发送者和接收者之间共享一个 VecDeque，用 Mutex 互斥，用 Condvar 通知
/// 同时，我们记录有多少个 senders 和 receivers

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Sender<T> {
    /// 生产者写入一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow!("no receivers left!"));
        }
        // 加锁访问， 压入数据之后立刻释放锁
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };
        // if was_empty => 可能会有挂起的 receiver, notify 一个rx来拿数据
        if was_empty {
            self.shared.available.notify_one();
        }
        Ok(())
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // 这里 sender deop 之后， 记得通知下潜在的阻塞 receiver
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if old <= 1 {
            self.shared.available.notify_all();
        }
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(v) = self.cache.pop_front() {
            return Ok(v);
        }
        let mut inner: MutexGuard<'_, VecDeque<T>> = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(t) => {
                    // 这里的 cache 不会有饥饿问题， 因为这是 mpsc 而不是mpmc, 这里只有一个 consumer
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    return Ok(t);
                }
                None if self.total_senders() == 0 => return Err(anyhow!("no senders left")),
                None => {
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("lock poisoned!"))
                        .unwrap();
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}
// 因为 receiver 检查了 total senders， 所以要记得在 sender drop 的时候更新计数
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

// impl<T> Iterator for Receiver<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.recv().ok()
//     }
// }

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE),
        },
    )
}

// clone trait 需要修改 shared 的 sender / receiver count
// 每次 sender send 和 receiver recv 的时候都需要检查 shared 的 tx/rx count

const INITIAL_SIZE: usize = 32;

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    // 此处省略所有 test case
    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world!".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world!");
    }
    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();
        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        // 在这个测试里，数据到达的顺序是不确定的，所以我们排个序再 assert
        result.sort();

        assert_eq!(result, [1, 2, 3]);
    }
    #[test]
    fn receiver_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据，确保它和发送的数据一致
                assert_eq!(idx, i);
            }
            // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
        thread::sleep(Duration::from_millis(1));

        // 再次发送数据，唤醒消费者
        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        // 留点时间让 receiver 处理
        thread::sleep(Duration::from_millis(1));

        // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
        assert_eq!(s1.total_queued_items(), 0);
    }
    #[test]
    fn last_sender_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        // sender 即用即抛
        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
                // sender 在此被丢弃
            })
            .join()
            .unwrap();
        }

        // 虽然没有 sender 了，接收者依然可以接受已经在队列里的数据
        for _ in 0..total {
            r.recv().unwrap();
        }

        // 然而，读取更多数据时会出错
        assert!(r.recv().is_err());
    }
    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        // 用于两个线程同步
        let (mut sender, mut receiver) = unbounded::<usize>();
        let t1 = thread::spawn(move || {
            // 保证 r.recv() 先于 t2 的 drop 执行
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
            println!("receiver dropped")
        });
        thread::sleep(Duration::from_millis(1));
        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
            println!("sender dropped")
        });
        t1.join().unwrap();
    }

    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();
        for i in 0..10usize {
            s.send(i).unwrap();
        }
        assert!(r.cache.is_empty());
        // 读取一个数据， 此行为会导致 swap, cache 中有数据
        assert_eq!(0, r.recv().unwrap());
        // 还有 9 个数据在 cache 中
        assert_eq!(r.cache.len(), 9);
        // 在 queue 里没有数据了
        assert_eq!(s.total_queued_items(), 0);
        // 从 cache 里读取剩下的数据
        for (idx, i) in r.into_iter().take(9).enumerate() {
            assert_eq!(idx + 1, i);
        }
    }
}
