//! Kafka Messaging Demo - 基于 rdkafka 的消息队列
//!
//! 演示: Producer、Consumer、结构化消息、Headers、批量发送、消费者组

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::{Header, Headers, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info, warn};

// ========== 消息模型 ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEvent {
    pub order_id: String,
    pub user_id: u64,
    pub product: String,
    pub amount: f64,
    pub event_type: String,
    pub timestamp: String,
}

// ========== Producer ==========

fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("compression.type", "snappy")
        .create()
        .expect("创建 Producer 失败")
}

async fn send_message(producer: &FutureProducer, topic: &str, key: &str, payload: &str) -> Result<(i32, i64), String> {
    let record = FutureRecord::to(topic).key(key).payload(payload);
    match producer.send(record, Duration::from_secs(5)).await {
        Ok((p, o)) => { info!("发送成功: topic={}, partition={}, offset={}", topic, p, o); Ok((p, o)) }
        Err((e, _)) => { error!("发送失败: {}", e); Err(e.to_string()) }
    }
}

async fn send_order_event(producer: &FutureProducer, topic: &str, event: &OrderEvent) -> Result<(), String> {
    let payload = serde_json::to_string(event).map_err(|e| e.to_string())?;
    let headers = OwnedHeaders::new()
        .insert(Header { key: "event_type", value: Some(&event.event_type) })
        .insert(Header { key: "source", value: Some("order-service") });
    let record = FutureRecord::to(topic).key(&event.order_id).payload(&payload).headers(headers);
    match producer.send(record, Duration::from_secs(5)).await {
        Ok((p, o)) => { info!("订单事件: order={}, type={}, p={}, o={}", event.order_id, event.event_type, p, o); Ok(()) }
        Err((e, _)) => Err(e.to_string()),
    }
}

// ========== Consumer ==========

fn create_consumer(brokers: &str, group: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "false")
        .set("session.timeout.ms", "10000")
        .create()
        .expect("创建 Consumer 失败")
}

async fn consume_messages(consumer: &StreamConsumer, topics: &[&str], max: usize) -> Vec<String> {
    use futures::StreamExt;
    consumer.subscribe(topics).expect("订阅失败");
    info!("开始消费: {:?}", topics);

    let mut received = Vec::new();
    let mut stream = consumer.stream();
    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(msg) = stream.next() => {
                match msg {
                    Ok(m) => {
                        let payload = m.payload_view::<str>().unwrap_or(Ok("")).unwrap_or("");
                        let key = m.key_view::<str>().unwrap_or(Ok("")).unwrap_or("");
                        info!("收到: topic={}, p={}, o={}, key={}", m.topic(), m.partition(), m.offset(), key);
                        if let Some(headers) = m.headers() {
                            for h in headers.iter() {
                                let v = h.value.map(|b| String::from_utf8_lossy(b).to_string()).unwrap_or_default();
                                info!("  Header: {}={}", h.key, v);
                            }
                        }
                        received.push(payload.to_string());
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                        if received.len() >= max { break; }
                    }
                    Err(e) => warn!("消费错误: {}", e),
                }
            }
            _ = &mut timeout => { info!("消费超时, 收到{}条", received.len()); break; }
        }
    }
    received
}

// ========== 演示 ==========

async fn demo_basic(brokers: &str) {
    println!("\n=== 1. 基础消息 ===");
    let producer = create_producer(brokers);
    for i in 1..=5 {
        let _ = send_message(&producer, "demo-basic", &format!("k{}", i), &format!("Hello Kafka #{}", i)).await;
    }
    println!("发送 5 条消息");

    println!("\n=== 2. 消费消息 ===");
    let consumer = create_consumer(brokers, "demo-basic-grp");
    let msgs = consume_messages(&consumer, &["demo-basic"], 5).await;
    println!("消费到 {} 条:", msgs.len());
    for (i, m) in msgs.iter().enumerate() { println!("  [{}] {}", i + 1, m); }
}

async fn demo_orders(brokers: &str) {
    println!("\n=== 3. 订单事件流 ===");
    let producer = create_producer(brokers);
    let events = vec![
        OrderEvent { order_id: "ORD-001".into(), user_id: 1001, product: "Rust编程之道".into(), amount: 99.0, event_type: "Created".into(), timestamp: chrono::Utc::now().to_rfc3339() },
        OrderEvent { order_id: "ORD-001".into(), user_id: 1001, product: "Rust编程之道".into(), amount: 99.0, event_type: "Paid".into(), timestamp: chrono::Utc::now().to_rfc3339() },
        OrderEvent { order_id: "ORD-002".into(), user_id: 1002, product: "Tokio实战".into(), amount: 79.0, event_type: "Created".into(), timestamp: chrono::Utc::now().to_rfc3339() },
    ];
    for e in &events { let _ = send_order_event(&producer, "demo-orders", e).await; }
    println!("发送 {} 个订单事件", events.len());

    let consumer = create_consumer(brokers, "demo-orders-grp");
    let msgs = consume_messages(&consumer, &["demo-orders"], events.len()).await;
    println!("消费到 {} 个事件:", msgs.len());
    for m in &msgs {
        if let Ok(e) = serde_json::from_str::<OrderEvent>(m) {
            println!("  {} - {} ({}元)", e.order_id, e.event_type, e.amount);
        }
    }
}

async fn demo_batch(brokers: &str) {
    println!("\n=== 4. 批量发送性能 ===");
    let producer = create_producer(brokers);
    let count = 1000u64;
    let start = std::time::Instant::now();

    let mut handles = Vec::new();
    for i in 0..count {
        let p = producer.clone();
        handles.push(tokio::spawn(async move {
            let key = format!("b{}", i);
            let payload = format!("msg-{}", i);
            let record = FutureRecord::to("demo-batch").key(&key).payload(&payload);
            p.send(record, Duration::from_secs(5)).await
        }));
    }
    let (mut ok, mut fail) = (0u64, 0u64);
    for h in handles { match h.await { Ok(Ok(_)) => ok += 1, _ => fail += 1 } }
    let elapsed = start.elapsed();
    println!("发送{}条: 成功={}, 失败={}, 耗时={:?}, QPS={:.0}", count, ok, fail, elapsed, count as f64 / elapsed.as_secs_f64());
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_env_filter("kafka_messaging=debug,rdkafka=warn").init();
    let config = shared::AppConfig::from_env();
    info!("Kafka Demo 启动, brokers={}", config.kafka_brokers);

    match ClientConfig::new().set("bootstrap.servers", &config.kafka_brokers).set("message.timeout.ms", "3000").create::<FutureProducer>() {
        Ok(_) => info!("Kafka 连接成功"),
        Err(e) => { error!("Kafka连接失败: {}. 请 docker-compose up -d", e); return; }
    }

    demo_basic(&config.kafka_brokers).await;
    demo_orders(&config.kafka_brokers).await;
    demo_batch(&config.kafka_brokers).await;
    info!("Kafka Demo 完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_kafka_available(brokers: &str) -> bool {
        ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "2000")
            .create::<FutureProducer>()
            .is_ok()
    }

    macro_rules! skip_if_no_kafka {
        ($brokers:expr) => {
            if !is_kafka_available($brokers) {
                eprintln!("跳过: Kafka 不可用");
                return;
            }
        };
    }

    // ========== 序列化测试 (无需 Kafka) ==========

    #[test]
    fn test_order_event_serialize() {
        let event = OrderEvent {
            order_id: "ORD-001".into(),
            user_id: 1001,
            product: "Rust Book".into(),
            amount: 99.0,
            event_type: "Created".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("ORD-001"));
        assert!(json.contains("1001"));
    }

    #[test]
    fn test_order_event_deserialize() {
        let json = r#"{"order_id":"ORD-002","user_id":1002,"product":"test","amount":50.0,"event_type":"Paid","timestamp":"2024-01-01"}"#;
        let event: OrderEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.order_id, "ORD-002");
        assert_eq!(event.user_id, 1002);
        assert_eq!(event.event_type, "Paid");
    }

    #[test]
    fn test_order_event_roundtrip() {
        let event = OrderEvent {
            order_id: "ORD-RT".into(),
            user_id: 999,
            product: "测试商品".into(),
            amount: 123.45,
            event_type: "Shipped".into(),
            timestamp: "2024-06-15T12:00:00Z".into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        let decoded: OrderEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.order_id, event.order_id);
        assert_eq!(decoded.amount, event.amount);
    }

    // ========== 集成测试 (需要 Kafka) ==========

    #[tokio::test]
    async fn test_send_and_consume() {
        let brokers = "localhost:9092";
        skip_if_no_kafka!(brokers);

        let producer = create_producer(brokers);
        let topic = "test-send-consume";

        // 发送
        for i in 0..3 {
            let result = send_message(&producer, topic, &format!("k{}", i), &format!("msg{}", i)).await;
            assert!(result.is_ok());
        }

        // 消费
        let group = format!("test-grp-{}", uuid::Uuid::new_v4());
        let consumer = create_consumer(brokers, &group);
        let msgs = consume_messages(&consumer, &[topic], 3).await;
        assert_eq!(msgs.len(), 3);
    }

    #[tokio::test]
    async fn test_send_order_event_integration() {
        let brokers = "localhost:9092";
        skip_if_no_kafka!(brokers);

        let producer = create_producer(brokers);
        let event = OrderEvent {
            order_id: "TEST-001".into(),
            user_id: 1,
            product: "test".into(),
            amount: 10.0,
            event_type: "Created".into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        let result = send_order_event(&producer, "test-orders", &event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_consumer_timeout_empty_topic() {
        let brokers = "localhost:9092";
        skip_if_no_kafka!(brokers);

        let group = format!("test-empty-{}", uuid::Uuid::new_v4());
        let consumer = create_consumer(brokers, &group);
        let topic = &format!("test-empty-{}", uuid::Uuid::new_v4());
        let msgs = consume_messages(&consumer, &[topic], 10).await;
        // 空 topic 应超时返回 0 条
        assert_eq!(msgs.len(), 0);
    }
}
