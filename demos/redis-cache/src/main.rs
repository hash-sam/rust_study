//! Redis Cache Demo - 基于 redis-rs 的缓存操作
//!
//! 演示: String/Hash/List/Set/ZSet、Pipeline、分布式锁、Cache Aside 模式

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
struct CachedUser { id: u64, name: String, email: String, score: f64 }

async fn connect(url: &str) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
    let client = redis::Client::open(url)?;
    let conn = client.get_multiplexed_async_connection().await?;
    info!("Redis 连接成功");
    Ok(conn)
}

// ========== 1. String ==========
async fn demo_string(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 1. String 基本操作 ===");
    let _: () = conn.set("greeting", "Hello Redis from Rust!").await?;
    let val: String = conn.get("greeting").await?;
    println!("SET/GET: {}", val);

    let _: () = conn.set_ex("temp", "临时数据", 60).await?;
    let ttl: i64 = redis::cmd("TTL").arg("temp").query_async(conn).await?;
    println!("SET EX: TTL={}s", ttl);

    let _: () = conn.set("views", 0i64).await?;
    let _: () = conn.incr("views", 7i64).await?;
    let views: i64 = conn.get("views").await?;
    println!("INCR: views={}", views);

    let s1: bool = conn.set_nx("uniq", "first").await?;
    let s2: bool = conn.set_nx("uniq", "second").await?;
    println!("SETNX: 1st={}, 2nd={}", s1, s2);
    Ok(())
}

// ========== 2. Hash ==========
async fn demo_hash(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 2. Hash 操作 ===");
    let k = "user:1001";
    let _: () = conn.hset(k, "name", "张三").await?;
    let _: () = conn.hset(k, "email", "zs@example.com").await?;
    let _: () = conn.hset(k, "score", "90").await?;
    let name: String = conn.hget(k, "name").await?;
    println!("HGET name={}", name);
    let all: Vec<(String, String)> = conn.hgetall(k).await?;
    println!("HGETALL: {:?}", all);
    let _: () = conn.hincr(k, "score", 5.5f64).await?;
    let score: f64 = conn.hget(k, "score").await?;
    println!("HINCRBY score={}", score);
    Ok(())
}

// ========== 3. List ==========
async fn demo_list(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 3. List 操作(队列) ===");
    let k = "queue";
    let _: () = conn.del(k).await?;
    for t in &["任务A", "任务B", "任务C"] { let _: () = conn.lpush(k, *t).await?; }
    let all: Vec<String> = conn.lrange(k, 0, -1).await?;
    println!("队列: {:?}", all);
    let popped: String = conn.rpop(k, None).await?;
    println!("RPOP: {}", popped);
    let len: i64 = conn.llen(k).await?;
    println!("剩余: {}", len);
    Ok(())
}

// ========== 4. Set ==========
async fn demo_set(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 4. Set 操作 ===");
    let _: () = conn.del("s1").await?;
    let _: () = conn.del("s2").await?;
    let _: () = conn.sadd("s1", &["rust", "tokio", "web"]).await?;
    let _: () = conn.sadd("s2", &["rust", "python", "web", "ai"]).await?;
    let inter: Vec<String> = conn.sinter(("s1", "s2")).await?;
    let union: Vec<String> = conn.sunion(("s1", "s2")).await?;
    println!("交集: {:?}", inter);
    println!("并集: {:?}", union);
    Ok(())
}

// ========== 5. Sorted Set ==========
async fn demo_zset(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 5. Sorted Set(排行榜) ===");
    let k = "leaderboard";
    let _: () = conn.del(k).await?;
    let _: () = conn.zadd(k, "张三", 92.5f64).await?;
    let _: () = conn.zadd(k, "李四", 88.0f64).await?;
    let _: () = conn.zadd(k, "王五", 95.0f64).await?;
    let _: () = conn.zadd(k, "赵六", 91.0f64).await?;
    let top: Vec<(String, f64)> = conn.zrevrange_withscores(k, 0, 2).await?;
    println!("Top 3:");
    for (i, (name, score)) in top.iter().enumerate() {
        println!("  {}. {} - {}分", i + 1, name, score);
    }
    let _: () = conn.zincr(k, "李四", 10.0f64).await?;
    let s: f64 = conn.zscore(k, "李四").await?;
    println!("李四+10分后: {}", s);
    Ok(())
}

// ========== 6. Pipeline ==========
async fn demo_pipeline(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 6. Pipeline 批量操作 ===");
    let t = std::time::Instant::now();
    let mut pipe = redis::pipe();
    for i in 0..100 { pipe.set(format!("pk:{}", i), format!("v{}", i)); }
    let _: () = pipe.query_async(conn).await?;
    println!("写入100个key: {:?}", t.elapsed());

    let t = std::time::Instant::now();
    let mut pipe = redis::pipe();
    for i in 0..100 { pipe.get(format!("pk:{}", i)); }
    let vals: Vec<String> = pipe.query_async(conn).await?;
    println!("读取100个key: {:?} (前3: {:?})", t.elapsed(), &vals[..3]);

    let mut pipe = redis::pipe();
    for i in 0..100 { pipe.del(format!("pk:{}", i)); }
    let _: () = pipe.query_async(conn).await?;
    Ok(())
}

// ========== 7. 分布式锁 ==========
struct DistLock { key: String, val: String, ttl: u64 }

impl DistLock {
    fn new(resource: &str, ttl: u64) -> Self {
        Self { key: format!("lock:{}", resource), val: uuid::Uuid::new_v4().to_string(), ttl }
    }
    async fn acquire(&self, conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<bool> {
        let r: Option<String> = redis::cmd("SET").arg(&self.key).arg(&self.val)
            .arg("NX").arg("EX").arg(self.ttl).query_async(conn).await?;
        Ok(r.is_some())
    }
    async fn release(&self, conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<bool> {
        let script = redis::Script::new(
            "if redis.call('GET',KEYS[1])==ARGV[1] then return redis.call('DEL',KEYS[1]) else return 0 end"
        );
        let r: i32 = script.key(&self.key).arg(&self.val).invoke_async(conn).await?;
        Ok(r == 1)
    }
}

async fn demo_lock(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 7. 分布式锁 ===");
    let l1 = DistLock::new("res", 10);
    println!("获取锁: {}", l1.acquire(conn).await?);
    let l2 = DistLock::new("res", 10);
    println!("再次获取(应失败): {}", l2.acquire(conn).await?);
    println!("释放锁: {}", l1.release(conn).await?);
    println!("释放后获取: {}", l2.acquire(conn).await?);
    let _ = l2.release(conn).await;
    Ok(())
}

// ========== 8. Cache Aside ==========
async fn demo_cache_aside(conn: &mut redis::aio::MultiplexedConnection) -> redis::RedisResult<()> {
    println!("\n=== 8. Cache Aside 模式 ===");
    let ck = "cache:user:1001";
    let cached: Option<String> = conn.get(ck).await?;
    if let Some(d) = cached {
        println!("缓存命中: {}", d);
    } else {
        println!("缓存未命中, 查询DB...");
        let user = CachedUser { id: 1001, name: "张三".into(), email: "zs@ex.com".into(), score: 95.5 };
        let json = serde_json::to_string(&user).unwrap();
        let _: () = conn.set_ex(ck, &json, 300).await?;
        println!("写入缓存(TTL=300s)");
    }
    let cached: Option<String> = conn.get(ck).await?;
    if let Some(d) = cached {
        let u: CachedUser = serde_json::from_str(&d).unwrap();
        println!("第二次查询 - 命中: {:?}", u);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_env_filter("redis_cache=debug").init();
    let config = shared::AppConfig::from_env();
    info!("Redis Cache Demo 启动, url={}", config.redis_url);

    let mut conn = match connect(&config.redis_url).await {
        Ok(c) => c,
        Err(e) => { error!("Redis连接失败: {}. 请 docker-compose up -d", e); return; }
    };
    let _: redis::RedisResult<()> = redis::cmd("FLUSHDB").query_async(&mut conn).await;

    macro_rules! run_demo {
        ($name:expr, $f:expr, $conn:expr) => {
            if let Err(e) = $f($conn).await {
                error!("{} 操作失败: {}", $name, e);
            }
        };
    }

    run_demo!("String", demo_string, &mut conn);
    run_demo!("Hash", demo_hash, &mut conn);
    run_demo!("List", demo_list, &mut conn);
    run_demo!("Set", demo_set, &mut conn);
    run_demo!("ZSet", demo_zset, &mut conn);
    run_demo!("Pipeline", demo_pipeline, &mut conn);
    run_demo!("Lock", demo_lock, &mut conn);
    run_demo!("CacheAside", demo_cache_aside, &mut conn);
    info!("Redis Cache Demo 完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_conn() -> Option<redis::aio::MultiplexedConnection> {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        connect(&url).await.ok()
    }

    macro_rules! skip_if_no_redis {
        ($conn:expr) => {
            match $conn {
                Some(c) => c,
                None => { eprintln!("跳过: Redis 不可用"); return; }
            }
        };
    }

    #[tokio::test]
    async fn test_string_set_get() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.set("test:str", "hello").await.unwrap();
        let val: String = conn.get("test:str").await.unwrap();
        assert_eq!(val, "hello");
        let _: () = conn.del("test:str").await.unwrap();
    }

    #[tokio::test]
    async fn test_incr() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.set("test:cnt", 0i64).await.unwrap();
        let _: () = conn.incr("test:cnt", 5i64).await.unwrap();
        let val: i64 = conn.get("test:cnt").await.unwrap();
        assert_eq!(val, 5);
        let _: () = conn.del("test:cnt").await.unwrap();
    }

    #[tokio::test]
    async fn test_setnx() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.del("test:nx").await.unwrap();
        let ok1: bool = conn.set_nx("test:nx", "first").await.unwrap();
        let ok2: bool = conn.set_nx("test:nx", "second").await.unwrap();
        assert!(ok1);
        assert!(!ok2);
        let val: String = conn.get("test:nx").await.unwrap();
        assert_eq!(val, "first");
        let _: () = conn.del("test:nx").await.unwrap();
    }

    #[tokio::test]
    async fn test_hash_ops() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let k = "test:hash";
        let _: () = conn.del(k).await.unwrap();
        let _: () = conn.hset(k, "name", "test").await.unwrap();
        let _: () = conn.hset(k, "age", "30").await.unwrap();
        let name: String = conn.hget(k, "name").await.unwrap();
        assert_eq!(name, "test");
        let exists: bool = conn.hexists(k, "name").await.unwrap();
        assert!(exists);
        let no: bool = conn.hexists(k, "phone").await.unwrap();
        assert!(!no);
        let _: () = conn.del(k).await.unwrap();
    }

    #[tokio::test]
    async fn test_list_ops() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let k = "test:list";
        let _: () = conn.del(k).await.unwrap();
        let _: () = conn.lpush(k, "a").await.unwrap();
        let _: () = conn.lpush(k, "b").await.unwrap();
        let _: () = conn.lpush(k, "c").await.unwrap();
        let len: i64 = conn.llen(k).await.unwrap();
        assert_eq!(len, 3);
        let popped: String = conn.rpop(k, None).await.unwrap();
        assert_eq!(popped, "a"); // FIFO
        let _: () = conn.del(k).await.unwrap();
    }

    #[tokio::test]
    async fn test_set_ops() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.del("test:s1").await.unwrap();
        let _: () = conn.del("test:s2").await.unwrap();
        let _: () = conn.sadd("test:s1", &["a", "b", "c"]).await.unwrap();
        let _: () = conn.sadd("test:s2", &["b", "c", "d"]).await.unwrap();
        let inter: Vec<String> = conn.sinter(("test:s1", "test:s2")).await.unwrap();
        assert_eq!(inter.len(), 2); // b, c
        let is: bool = conn.sismember("test:s1", "a").await.unwrap();
        assert!(is);
        let _: () = conn.del("test:s1").await.unwrap();
        let _: () = conn.del("test:s2").await.unwrap();
    }

    #[tokio::test]
    async fn test_sorted_set() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let k = "test:zset";
        let _: () = conn.del(k).await.unwrap();
        let _: () = conn.zadd(k, "alice", 90.0f64).await.unwrap();
        let _: () = conn.zadd(k, "bob", 85.0f64).await.unwrap();
        let _: () = conn.zadd(k, "carol", 95.0f64).await.unwrap();
        let top: Vec<(String, f64)> = conn.zrevrange_withscores(k, 0, 0).await.unwrap();
        assert_eq!(top[0].0, "carol");
        assert_eq!(top[0].1, 95.0);
        let _: () = conn.del(k).await.unwrap();
    }

    #[tokio::test]
    async fn test_pipeline() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let mut pipe = redis::pipe();
        for i in 0..10 {
            pipe.set(format!("test:pipe:{}", i), format!("v{}", i));
        }
        let _: () = pipe.query_async(&mut conn).await.unwrap();

        let mut pipe = redis::pipe();
        for i in 0..10 { pipe.get(format!("test:pipe:{}", i)); }
        let vals: Vec<String> = pipe.query_async(&mut conn).await.unwrap();
        assert_eq!(vals.len(), 10);
        assert_eq!(vals[0], "v0");
        assert_eq!(vals[9], "v9");

        let mut pipe = redis::pipe();
        for i in 0..10 { pipe.del(format!("test:pipe:{}", i)); }
        let _: () = pipe.query_async(&mut conn).await.unwrap();
    }

    #[tokio::test]
    async fn test_distributed_lock() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.del("lock:test_res").await.unwrap();

        let l1 = DistLock::new("test_res", 10);
        assert!(l1.acquire(&mut conn).await.unwrap());

        let l2 = DistLock::new("test_res", 10);
        assert!(!l2.acquire(&mut conn).await.unwrap());

        assert!(l1.release(&mut conn).await.unwrap());
        assert!(l2.acquire(&mut conn).await.unwrap());
        l2.release(&mut conn).await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_aside_pattern() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let ck = "test:cache:user:99";
        let _: () = conn.del(ck).await.unwrap();

        // Miss
        let cached: Option<String> = conn.get(ck).await.unwrap();
        assert!(cached.is_none());

        // 写入缓存
        let user = CachedUser { id: 99, name: "test".into(), email: "t@t.com".into(), score: 80.0 };
        let json = serde_json::to_string(&user).unwrap();
        let _: () = conn.set_ex(ck, &json, 60).await.unwrap();

        // Hit
        let cached: String = conn.get(ck).await.unwrap();
        let u: CachedUser = serde_json::from_str(&cached).unwrap();
        assert_eq!(u.id, 99);
        assert_eq!(u.name, "test");

        let _: () = conn.del(ck).await.unwrap();
    }

    #[tokio::test]
    async fn test_ttl_expiry() {
        let mut conn = skip_if_no_redis!(get_test_conn().await);
        let _: () = conn.set_ex("test:ttl", "val", 1).await.unwrap();
        let val: Option<String> = conn.get("test:ttl").await.unwrap();
        assert!(val.is_some());
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        let val: Option<String> = conn.get("test:ttl").await.unwrap();
        assert!(val.is_none());
    }
}
