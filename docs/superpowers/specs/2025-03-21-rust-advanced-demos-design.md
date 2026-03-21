# Rust Advanced Demos 设计文档

**日期**: 2025-03-21
**项目**: rust_study
**目标**: 为 Rust 学习项目添加生产级实际应用 demo

---

## 1. 项目概述

### 1.1 目标

在现有的 Rust 基础学习项目基础上，添加一系列生产级应用 demo，展示 Rust 在实际场景中的应用，包括：
- HTTP 服务开发
- MySQL 数据库操作
- Redis 缓存
- Kafka 消息队列
- 高并发处理
- 完整的集成应用

### 1.2 学习目标定位

**完整的生产级应用**：构建接近生产质量的代码，展示 Rust 在实际项目中的应用，包括完整的错误处理、日志、监控等。

### 1.3 组织方式

**独立的 workspace 成员**：每个 demo 都是独立的 Rust crate，有自己的 Cargo.toml，可以单独运行。适合独立学习每个技术栈。

---

## 2. 项目结构

```
rust_study/
├── Cargo.toml                    # Workspace 配置
├── src/                          # 现有基础学习内容（保持不变）
├── demos/                        # 新的深入 demo 目录
│   ├── http-api/                 # Demo 1: RESTful API 服务
│   ├── mysql-crud/               # Demo 2: MySQL 数据库操作
│   ├── redis-cache/              # Demo 3: Redis 缓存服务
│   ├── kafka-messaging/          # Demo 4: Kafka 消息队列
│   ├── concurrent-tasks/         # Demo 5: 并发任务处理
│   └── integrated-service/       # Demo 6: 集成服务（综合所有技术）
├── docs/
│   └── superpowers/
│       └── specs/
│           └── 2025-03-21-rust-advanced-demos-design.md
└── README.md
```

---

## 3. 技术栈

### 3.1 核心依赖

```toml
[workspace.dependencies]
# 异步运行时
tokio = { version = "1.40", features = ["full"] }
# HTTP 框架
axum = "0.8"
# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# 数据库
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql", "chrono", "uuid"] }
# Redis
redis = { version = "0.26", features = ["tokio-comp", "connection-manager"] }
# Kafka
rdkafka = { version = "0.36", features = ["tokio"] }
# 工具库
anyhow = "1.0"
thiserror = "2.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
config = "0.14"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
# 测试
tokio-test = "0.4"
```

### 3.2 技术选择

| 组件 | 选择 | 原因 |
|------|------|------|
| HTTP 框架 | Axum | 现代、类型安全、基于 Tower 生态，与 Tokio 集成良好 |
| 异步运行时 | Tokio | Rust 事实上的标准运行时，生态系统支持最广泛 |
| MySQL 客户端 | SQLx | 编译时 SQL 验证，类型安全，性能优秀 |
| Redis 客户端 | redis-rs | 标准选择，tokio 支持 |
| Kafka 客户端 | rust-rdkafka | 性能最优，业界标准 |

---

## 4. 各 Demo 详细设计

### 4.1 Demo 1: http-api - RESTful API 服务

**功能：**
- 用户管理 API（CRUD）
- 健康检查端点
- 请求日志中间件
- 速率限制中间件
- 统一错误处理

**核心模型：**
```rust
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
```

**关键实现点：**
- Axum 路由器模块化
- 请求验证和错误响应
- 结构化日志（tracing）
- 集成测试

**目录结构：**
```
http-api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── routes/
│   ├── handlers/
│   ├── models/
│   ├── middleware/
│   ├── error.rs
│   └── config.rs
├── tests/
└── README.md
```

### 4.2 Demo 2: mysql-crud - 数据库操作

**功能：**
- 完整的 CRUD 操作
- 复杂查询（JOIN、聚合）
- 事务管理
- 批量操作
- 数据库连接池配置

**数据表设计：**
```sql
CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE posts (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

**关键实现点：**
- SQLx `.sql` 文件和编译时验证
- Repository 模式封装
- 事务处理示例
- 连接池配置和监控

**目录结构：**
```
mysql-crud/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── db/
│   ├── models/
│   ├── repository/
│   ├── migrations/
│   └── config.rs
├── tests/
├── sql/
│   ├── 001_create_users.sql
│   └── 002_create_posts.sql
└── README.md
```

### 4.3 Demo 3: redis-cache - 缓存服务

**功能：**
- 通用缓存接口
- 多种缓存策略（Cache Aside、Write Through、Write Back）
- 缓存预热
- 防穿透/雪崩/击穿

**缓存接口设计：**
```rust
#[async_trait]
pub trait Cache: Send + Sync {
    async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where T: DeserializeOwned;

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<()>;

    async fn delete(&self, key: &str) -> Result<()>;

    async fn exists(&self, key: &str) -> Result<bool>;
}
```

**关键实现点：**
- Redis 连接管理
- 序列化/反序列化
- 性能监控

**目录结构：**
```
redis-cache/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── redis_cache.rs
│   │   └── memory_cache.rs
│   ├── strategies/
│   └── config.rs
├── tests/
└── README.md
```

### 4.4 Demo 4: kafka-messaging - 消息队列

**功能：**
- Producer：发送各种类型的消息
- Consumer：消费消息并处理
- 消息序列化/反序列化
- 消费者组配置
- 重试机制和死信队列

**消息模型：**
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub timestamp: i64,
    pub event_type: String,
    pub payload: Value,
}
```

**关键实现点：**
- 异步发送和回调
- 消费者偏移量管理
- 错误处理和重试策略
- 幂等性处理

**目录结构：**
```
kafka-messaging/
├── Cargo.toml
├── src/
│   ├── main.rs           # 可选择运行 producer 或 consumer
│   ├── producer/
│   ├── consumer/
│   ├── models/
│   └── config.rs
├── tests/
└── README.md
```

### 4.5 Demo 5: concurrent-tasks - 并发处理

**模块结构：**

**channels/** - Channel 通信示例
- mpsc：多生产者单消费者
- broadcast：广播
- watch：多读者单写者
- oneshot：一次性通道

**shared-state/** - 共享状态
- Arc<Mutex> 互斥锁
- Arc<RwLock> 读写锁
- Arc<AtomicUsize> 原子类型
- Semaphore 信号量

**threadpools/** - 线程池
- Tokio 任务池
- Rayon 并行计算
- 自定义工作窃取线程池

**async-concurrency/** - 异步并发
- join! 并行执行
- select! 竞争执行
- 超时控制
- 取消令牌

**实战示例：**
- 并行下载器
- 并发爬虫
- 生产者-消费者模型
- 速率限制器
- 熔断器实现

**目录结构：**
```
concurrent-tasks/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── channels/
│   ├── shared_state/
│   ├── threadpools/
│   ├── sync_primitives/
│   └── async_concurrency/
├── benchmarks/
│   ├── bench_channels.rs
│   └── bench_concurrency.rs
└── README.md
```

### 4.6 Demo 6: integrated-service - 集成服务

**架构：**
```
┌─────────────────┐
│   HTTP API      │ ← Axum + 中间件
├─────────────────┤
│  Services Layer │ ← 业务逻辑
├─────────────────┤
│  Repositories   │ ← MySQL (SQLx)
├─────────────────┤
│     Cache       │ ← Redis
├─────────────────┤
│   Messaging     │ ← Kafka
└─────────────────┘
```

**完整功能：**
- 用户注册/登录（MySQL）
- 会话管理
- 帖子发布和浏览（MySQL + 缓存）
- 事件通知
- 限流和降级
- 优雅关闭

**Docker Compose 包含：**
- MySQL
- Redis
- Kafka/ZooKeeper
- 应用服务

**目录结构：**
```
integrated-service/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── api/
│   ├── services/
│   ├── repositories/
│   ├── cache/
│   ├── messaging/
│   ├── middleware/
│   ├── error.rs
│   └── config.rs
├── tests/
├── docker-compose.yml
└── README.md
```

---

## 5. 生产级特性

### 5.1 统一错误处理

```rust
#[derive(Debug)]
pub enum AppError {
    Database(String),
    Cache(String),
    Validation(String),
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Database(_) | AppError::Cache(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "服务暂时不可用".to_string())
            }
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        (status, Json(body)).into_response()
    }
}
```

### 5.2 结构化日志

使用 `tracing` 和 `tracing-subscriber` 实现结构化日志：
- 日志级别：ERROR、WARN、INFO、DEBUG、TRACE
- JSON 格式输出（生产环境）
- 请求 ID 追踪
- 上下文信息

### 5.3 配置管理

使用 `config` crate 支持多环境配置：
- YAML 配置文件
- 环境变量覆盖
- 配置验证

**配置文件示例：**
```yaml
server:
  host: "127.0.0.1"
  port: 3000

database:
  url: "mysql://root:password@localhost:3306/rust_study"
  max_connections: 10

redis:
  url: "redis://127.0.0.1:6379"
  max_connections: 10

kafka:
  bootstrap_servers: "localhost:9092"
  group_id: "rust_study_group"
```

### 5.4 健康检查

**健康检查端点：**
- `/health` - 服务健康状态
- `/readiness` - 就绪检查（依赖服务状态）

### 5.5 优雅关闭

- 监听 SIGTERM 和 SIGINT 信号
- 停止接受新请求
- 等待现有请求完成
- 关闭数据库连接
- 清理资源

### 5.6 测试策略

**测试层次：**
1. **单元测试**：测试单个函数和模块
2. **集成测试**：测试多个组件的交互
3. **端到端测试**：测试完整的 API 流程

**测试工具：**
- `tokio-test`：异步测试工具
- `axum-test`：HTTP 测试工具
- `tower`：服务测试工具

---

## 6. 实施计划

### 6.1 实施顺序

1. **第一阶段**：基础设施
   - 配置 workspace
   - 创建共享配置和错误处理模块

2. **第二阶段**：核心 Demo（按顺序）
   - http-api
   - mysql-crud
   - redis-cache
   - kafka-messaging
   - concurrent-tasks

3. **第三阶段**：集成应用
   - integrated-service
   - Docker Compose 配置

4. **第四阶段**：完善和文档
   - 完善测试覆盖
   - 编写详细文档
   - 性能基准测试

### 6.2 验收标准

每个 Demo 必须：
- ✅ 可以独立编译和运行
- ✅ 包含完整的错误处理
- ✅ 有结构化日志
- ✅ 有配置管理
- ✅ 有单元测试和集成测试
- ✅ 有 README 文档说明

---

## 7. 学习路径

**推荐学习顺序：**
1. 基础语法（现有内容）
2. http-api → 学习 HTTP 服务开发
3. mysql-crud → 学习数据库操作
4. redis-cache → 学习缓存使用
5. kafka-messaging → 学习消息队列
6. concurrent-tasks → 学习并发编程
7. integrated-service → 学习完整应用架构

**并发模式学习：**
- 基础：Channel、Arc+Mutex
- 进阶：线程池、任务调度
- 高级：无锁数据结构、Actor 模型

---

## 8. 后续扩展

可能的扩展方向：
- gRPC 服务
- WebSocket 实时通信
- GraphQL API
- 分布式追踪
- 服务网格集成
- 云原生部署

---

## 9. 总结

本设计为 rust_study 项目添加了 6 个生产级 demo，覆盖了现代 Rust Web 开发的核心技术栈。每个 demo 都是独立的 workspace 成员，可以单独学习和运行。通过循序渐进的学习路径，开发者可以掌握从简单的 HTTP API 到复杂的分布式系统的完整开发技能。
