//! Concurrent Tasks Demo - Rust 高并发编程模式
//!
//! 演示:
//! 1. tokio 异步任务 (spawn, join, select)
//! 2. 共享状态 (Arc + Mutex/RwLock)
//! 3. Channel 通信 (mpsc, oneshot, broadcast)
//! 4. 原子操作 (AtomicU64)
//! 5. Rayon 并行计算 (CPU 密集型)
//! 6. Semaphore 限流
//! 7. 并发实战: 网页爬虫模拟

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex, RwLock, Semaphore};
use tracing::info;

// ========== 1. tokio::spawn 异步任务 ==========

async fn demo_spawn() {
    println!("\n=== 1. tokio::spawn 异步并发 ===");
    let mut handles = Vec::new();

    for i in 0..5 {
        let h = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(100 - i * 10)).await;
            println!("  任务 {} 完成", i);
            i * 10
        });
        handles.push(h);
    }

    let mut results = Vec::new();
    for h in handles {
        results.push(h.await.unwrap());
    }
    println!("所有任务完成, 结果: {:?}", results);
}

// ========== 2. tokio::select! 竞争 ==========

async fn demo_select() {
    println!("\n=== 2. tokio::select! 竞争执行 ===");

    let fast = async {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        "快任务"
    };
    let slow = async {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        "慢任务"
    };

    // 只等最先完成的
    tokio::select! {
        result = fast => println!("胜出: {}", result),
        result = slow => println!("胜出: {}", result),
    }
}

// ========== 3. 共享状态 - Arc + Mutex ==========

async fn demo_shared_state() {
    println!("\n=== 3. 共享状态 (Arc+Mutex) ===");

    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let c = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..1000 {
                let mut val = c.lock().await;
                *val += 1;
            }
        }));
    }
    for h in handles { h.await.unwrap(); }
    println!("10个任务各+1000, 最终值: {}", *counter.lock().await);
}

// ========== 4. Arc + RwLock ==========

async fn demo_rwlock() {
    println!("\n=== 4. RwLock 读写分离 ===");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = Vec::new();

    // 多个并发读
    for i in 0..3 {
        let d = data.clone();
        handles.push(tokio::spawn(async move {
            let r = d.read().await;
            println!("  读者{}: {:?}", i, *r);
        }));
    }

    // 一个写入
    let d = data.clone();
    handles.push(tokio::spawn(async move {
        let mut w = d.write().await;
        w.push(4);
        w.push(5);
        println!("  写者: 添加了 4, 5");
    }));

    for h in handles { h.await.unwrap(); }
    println!("最终数据: {:?}", *data.read().await);
}

// ========== 5. 原子操作 ==========

async fn demo_atomic() {
    println!("\n=== 5. 原子操作 (无锁计数器) ===");

    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let c = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..10000 {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles { h.await.unwrap(); }
    println!("10个任务各+10000, 原子计数器: {}", counter.load(Ordering::Relaxed));
}

// ========== 6. mpsc Channel ==========

async fn demo_mpsc() {
    println!("\n=== 6. mpsc Channel (多生产者单消费者) ===");

    let (tx, mut rx) = mpsc::channel::<String>(32);

    // 多个生产者
    for i in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for j in 0..3 {
                tx.send(format!("生产者{}-消息{}", i, j)).await.unwrap();
            }
        });
    }
    drop(tx); // 关闭发送端

    // 单消费者
    let mut received = Vec::new();
    while let Some(msg) = rx.recv().await {
        received.push(msg);
    }
    println!("收到 {} 条消息: {:?}", received.len(), received);
}

// ========== 7. oneshot Channel ==========

async fn demo_oneshot() {
    println!("\n=== 7. oneshot Channel (请求-响应) ===");

    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        tx.send("计算结果: 42".to_string()).unwrap();
    });

    let result = rx.await.unwrap();
    println!("收到响应: {}", result);
}

// ========== 8. broadcast Channel ==========

async fn demo_broadcast() {
    println!("\n=== 8. broadcast Channel (广播) ===");

    let (tx, _) = broadcast::channel::<String>(16);
    let mut handles = Vec::new();

    // 3个订阅者
    for i in 0..3 {
        let mut rx = tx.subscribe();
        handles.push(tokio::spawn(async move {
            let mut msgs = Vec::new();
            while let Ok(msg) = rx.recv().await {
                msgs.push(msg);
            }
            println!("  订阅者{}: 收到 {:?}", i, msgs);
        }));
    }

    // 发送广播
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    tx.send("公告1".into()).unwrap();
    tx.send("公告2".into()).unwrap();
    drop(tx);

    for h in handles { h.await.unwrap(); }
}

// ========== 9. Semaphore 限流 ==========

async fn demo_semaphore() {
    println!("\n=== 9. Semaphore 并发限流 ===");

    let sem = Arc::new(Semaphore::new(3)); // 最多3个并发
    let mut handles = Vec::new();
    let active = Arc::new(AtomicU64::new(0));

    for i in 0..10 {
        let sem = sem.clone();
        let active = active.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let cur = active.fetch_add(1, Ordering::Relaxed) + 1;
            println!("  任务{} 开始 (并发数: {})", i, cur);
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            active.fetch_sub(1, Ordering::Relaxed);
        }));
    }
    for h in handles { h.await.unwrap(); }
    println!("10个任务完成, 最大并发=3");
}

// ========== 10. Rayon 并行计算 ==========

fn demo_rayon() {
    println!("\n=== 10. Rayon CPU并行计算 ===");
    use rayon::prelude::*;

    let data: Vec<u64> = (1..=10_000_000).collect();

    // 串行
    let t = std::time::Instant::now();
    let sum_serial: u64 = data.iter().sum();
    let d1 = t.elapsed();

    // 并行
    let t = std::time::Instant::now();
    let sum_parallel: u64 = data.par_iter().sum();
    let d2 = t.elapsed();

    println!("串行求和: {} ({:?})", sum_serial, d1);
    println!("并行求和: {} ({:?})", sum_parallel, d2);
    println!("加速比: {:.2}x", d1.as_secs_f64() / d2.as_secs_f64());

    // 并行 map-reduce
    let t = std::time::Instant::now();
    let count = data.par_iter().filter(|&&x| x % 7 == 0 || x % 13 == 0).count();
    let d = t.elapsed();
    println!("并行filter(能被7或13整除): {} 个 ({:?})", count, d);
}

// ========== 11. 并发爬虫模拟 ==========

async fn demo_concurrent_crawler() {
    println!("\n=== 11. 并发爬虫模拟 ===");

    let urls: Vec<String> = (1..=20).map(|i| format!("https://example.com/page/{}", i)).collect();
    let sem = Arc::new(Semaphore::new(5)); // 限制5个并发
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();
    for url in urls {
        let sem = sem.clone();
        let results = results.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // 模拟网络请求
            let delay = rand::random::<u64>() % 100 + 20;
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            let size = rand::random::<usize>() % 10000 + 1000;
            results.lock().await.push((url, size, delay));
        }));
    }
    for h in handles { h.await.unwrap(); }

    let elapsed = start.elapsed();
    let results = results.lock().await;
    let total_size: usize = results.iter().map(|(_, s, _)| s).sum();
    println!("爬取 {} 个页面, 总大小: {} bytes, 耗时: {:?}", results.len(), total_size, elapsed);
    println!("(并发度=5, 如串行约需 {}ms)", results.iter().map(|(_, _, d)| d).sum::<u64>());
}

// ========== main ==========

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_env_filter("concurrent_tasks=debug").init();
    info!("Concurrent Tasks Demo 启动");

    demo_spawn().await;
    demo_select().await;
    demo_shared_state().await;
    demo_rwlock().await;
    demo_atomic().await;
    demo_mpsc().await;
    demo_oneshot().await;
    demo_broadcast().await;
    demo_semaphore().await;
    demo_rayon();
    demo_concurrent_crawler().await;

    info!("Concurrent Tasks Demo 完成");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use tokio::sync::{mpsc, oneshot, broadcast, Mutex, RwLock, Semaphore};

    // ========== spawn 测试 ==========

    #[tokio::test]
    async fn test_spawn_all_complete() {
        let mut handles = Vec::new();
        for i in 0..10 {
            handles.push(tokio::spawn(async move { i * 2 }));
        }
        let mut results = Vec::new();
        for h in handles {
            results.push(h.await.unwrap());
        }
        assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    // ========== select 测试 ==========

    #[tokio::test]
    async fn test_select_fastest_wins() {
        let result = tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(10)) => "fast",
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => "slow",
        };
        assert_eq!(result, "fast");
    }

    // ========== Mutex 测试 ==========

    #[tokio::test]
    async fn test_mutex_concurrent_increment() {
        let counter = Arc::new(Mutex::new(0u64));
        let mut handles = Vec::new();
        for _ in 0..10 {
            let c = counter.clone();
            handles.push(tokio::spawn(async move {
                for _ in 0..100 {
                    *c.lock().await += 1;
                }
            }));
        }
        for h in handles { h.await.unwrap(); }
        assert_eq!(*counter.lock().await, 1000);
    }

    // ========== RwLock 测试 ==========

    #[tokio::test]
    async fn test_rwlock_concurrent_reads() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3]));
        let mut handles = Vec::new();
        // 多个并发读不阻塞
        for _ in 0..10 {
            let d = data.clone();
            handles.push(tokio::spawn(async move {
                let r = d.read().await;
                assert_eq!(r.len(), 3);
            }));
        }
        for h in handles { h.await.unwrap(); }
    }

    #[tokio::test]
    async fn test_rwlock_write_then_read() {
        let data = Arc::new(RwLock::new(vec![1]));
        data.write().await.push(2);
        data.write().await.push(3);
        let r = data.read().await;
        assert_eq!(*r, vec![1, 2, 3]);
    }

    // ========== Atomic 测试 ==========

    #[tokio::test]
    async fn test_atomic_concurrent() {
        let counter = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();
        for _ in 0..10 {
            let c = counter.clone();
            handles.push(tokio::spawn(async move {
                for _ in 0..1000 {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles { h.await.unwrap(); }
        assert_eq!(counter.load(Ordering::Relaxed), 10000);
    }

    // ========== mpsc Channel 测试 ==========

    #[tokio::test]
    async fn test_mpsc_multi_producer() {
        let (tx, mut rx) = mpsc::channel::<i32>(32);
        for i in 0..5 {
            let tx = tx.clone();
            tokio::spawn(async move { tx.send(i).await.unwrap(); });
        }
        drop(tx);
        let mut received = Vec::new();
        while let Some(v) = rx.recv().await { received.push(v); }
        received.sort();
        assert_eq!(received, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_mpsc_bounded_capacity() {
        let (tx, mut rx) = mpsc::channel::<i32>(2);
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        // 缓冲区满时 try_send 返回错误
        assert!(tx.try_send(3).is_err());
        assert_eq!(rx.recv().await, Some(1));
        // 消费一个后可以发送
        tx.send(3).await.unwrap();
        assert_eq!(rx.recv().await, Some(2));
        assert_eq!(rx.recv().await, Some(3));
    }

    // ========== oneshot Channel 测试 ==========

    #[tokio::test]
    async fn test_oneshot_request_response() {
        let (tx, rx) = oneshot::channel::<String>();
        tokio::spawn(async move { tx.send("result".to_string()).unwrap(); });
        assert_eq!(rx.await.unwrap(), "result");
    }

    #[tokio::test]
    async fn test_oneshot_dropped_sender() {
        let (tx, rx) = oneshot::channel::<String>();
        drop(tx);
        assert!(rx.await.is_err());
    }

    // ========== broadcast Channel 测试 ==========

    #[tokio::test]
    async fn test_broadcast_all_receive() {
        let (tx, _) = broadcast::channel::<String>(16);
        let mut rx1 = tx.subscribe();
        let mut rx2 = tx.subscribe();
        let mut rx3 = tx.subscribe();

        tx.send("hello".to_string()).unwrap();
        assert_eq!(rx1.recv().await.unwrap(), "hello");
        assert_eq!(rx2.recv().await.unwrap(), "hello");
        assert_eq!(rx3.recv().await.unwrap(), "hello");
    }

    // ========== Semaphore 测试 ==========

    #[tokio::test]
    async fn test_semaphore_limits_concurrency() {
        let sem = Arc::new(Semaphore::new(3));
        let active = Arc::new(AtomicU64::new(0));
        let max_active = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();

        for _ in 0..20 {
            let sem = sem.clone();
            let active = active.clone();
            let max_active = max_active.clone();
            handles.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let cur = active.fetch_add(1, Ordering::SeqCst) + 1;
                // 更新最大并发数
                max_active.fetch_max(cur, Ordering::SeqCst);
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                active.fetch_sub(1, Ordering::SeqCst);
            }));
        }
        for h in handles { h.await.unwrap(); }
        assert!(max_active.load(Ordering::SeqCst) <= 3);
    }

    // ========== Rayon 测试 ==========

    #[test]
    fn test_rayon_parallel_sum() {
        use rayon::prelude::*;
        let data: Vec<u64> = (1..=1000).collect();
        let serial_sum: u64 = data.iter().sum();
        let parallel_sum: u64 = data.par_iter().sum();
        assert_eq!(serial_sum, parallel_sum);
        assert_eq!(serial_sum, 500500);
    }

    #[test]
    fn test_rayon_parallel_filter() {
        use rayon::prelude::*;
        let data: Vec<u64> = (1..=100).collect();
        let serial: Vec<u64> = data.iter().filter(|&&x| x % 3 == 0).copied().collect();
        let parallel: Vec<u64> = data.par_iter().filter(|&&x| x % 3 == 0).copied().collect();
        assert_eq!(serial, parallel);
    }

    #[test]
    fn test_rayon_parallel_map_reduce() {
        use rayon::prelude::*;
        let data: Vec<i64> = (1..=100).collect();
        let result: i64 = data.par_iter().map(|&x| x * x).sum();
        assert_eq!(result, 338350); // sum of squares 1..100
    }

    // ========== 综合: 生产者消费者模式 ==========

    #[tokio::test]
    async fn test_producer_consumer_pattern() {
        let (tx, mut rx) = mpsc::channel::<u64>(100);

        // 多个生产者
        for i in 0..5 {
            let tx = tx.clone();
            tokio::spawn(async move {
                for j in 0..10 {
                    tx.send(i * 10 + j).await.unwrap();
                }
            });
        }
        drop(tx);

        // 消费者收集所有结果
        let mut results = Vec::new();
        while let Some(v) = rx.recv().await { results.push(v); }
        assert_eq!(results.len(), 50);
    }

    // ========== 超时控制测试 ==========

    #[tokio::test]
    async fn test_timeout() {
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(50),
            async {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                42
            },
        )
        .await;
        assert!(result.is_err()); // 超时
    }

    #[tokio::test]
    async fn test_timeout_success() {
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            async { 42 },
        )
        .await;
        assert_eq!(result.unwrap(), 42);
    }
}
