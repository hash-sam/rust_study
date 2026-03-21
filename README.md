# Rust Study - Rust 系统学习项目

从零到生产级的 Rust 学习路径，涵盖语法基础到实战 Demo（HTTP/MySQL/Redis/Kafka/高并发）。

## 项目结构

```
rust_study/
├── src/                          # 基础语法学习（交互式菜单）
│   ├── basics/                   # 变量、数据类型、函数
│   ├── control_flow/             # if-else、循环
│   ├── ownership/                # 所有权、借用
│   ├── structs_enums/            # 结构体、枚举、模式匹配
│   ├── collections/              # Vec、String、HashMap
│   ├── modules_errors/           # 模块系统、错误处理
│   └── advanced/                 # 泛型、Trait、生命周期
├── demos/                        # 生产级实战 Demo
│   ├── shared/                   # 公共模块（错误处理、配置管理）
│   ├── http-api/                 # Axum RESTful API
│   ├── mysql-crud/               # SQLx 数据库操作
│   ├── redis-cache/              # Redis 缓存
│   ├── kafka-messaging/          # Kafka 消息队列
│   ├── concurrent-tasks/         # 高并发模式
│   └── integrated-service/       # 整合微服务
└── docker-compose.yml            # MySQL/Redis/Kafka 本地环境
```

## 快速开始

```bash
# 运行基础语法学习（交互式菜单）
cargo run -p rust_study

# 启动外部依赖
docker-compose up -d

# 运行各 Demo
cargo run -p http-api              # HTTP 服务 → http://localhost:3000
cargo run -p mysql-crud            # MySQL CRUD 演示
cargo run -p redis-cache           # Redis 操作演示
cargo run -p kafka-messaging       # Kafka 消息演示
cargo run -p concurrent-tasks      # 高并发（无需外部依赖）
cargo run -p integrated-service    # 整合微服务 → http://localhost:3000
```

## 基础语法模块

交互式学习，`cargo run` 后选择数字运行对应主题：

| 编号 | 主题 | 核心知识点 |
|------|------|-----------|
| 1 | 基础语法 | 不可变/可变变量、shadowing、常量、基础类型、函数 |
| 2 | 流程控制 | if/else 表达式、for/while/loop、break 返回值 |
| 3 | 所有权与借用 | 移动语义、引用规则、可变引用、悬垂引用 |
| 4 | 结构体与枚举 | struct 方法、enum 关联数据、match/if let/while let |
| 5 | 集合 | Vec\<T\>、String vs &str、HashMap、BTreeMap |
| 6 | 模块与错误 | mod/pub/use、Result\<T,E\>、? 操作符、自定义错误 |
| 7 | 高级特性 | 泛型、Trait(静态/动态分发)、生命周期标注 |

## 实战 Demo 详解

### http-api — Axum RESTful 服务

```
GET    /health           # 健康检查
GET    /api/users        # 用户列表（支持 ?page=1&per_page=10）
POST   /api/users        # 创建用户 {"name":"..","email":"..","age":28}
GET    /api/users/:id    # 查询用户
PUT    /api/users/:id    # 更新用户
DELETE /api/users/:id    # 删除用户
```

**知识点**: 路由嵌套、`Arc<RwLock>` 状态共享、JSON 提取器、中间件(CORS/Trace)、统一错误处理(`IntoResponse`)、优雅关闭

### mysql-crud — SQLx 数据库操作

**知识点**: 连接池配置(`MySqlPoolOptions`)、`query_as` 类型安全映射、事务(`begin/commit`)、JOIN 查询、模糊搜索、聚合统计

### redis-cache — Redis 缓存

**知识点**: 五大数据类型(String/Hash/List/Set/ZSet)、Pipeline 批量操作、分布式锁(SET NX EX + Lua 脚本释放)、Cache Aside 模式、TTL 过期策略

### kafka-messaging — Kafka 消息队列

**知识点**: FutureProducer/StreamConsumer、结构化消息+Headers、消费者组、`tokio::select!` 超时消费、批量并发发送性能测试

### concurrent-tasks — 高并发模式

无需外部依赖，直接 `cargo run -p concurrent-tasks`：

| 模式 | 说明 |
|------|------|
| `tokio::spawn` | 异步任务并发执行 |
| `tokio::select!` | 竞争执行，最快者胜出 |
| `Arc<Mutex>` | 共享状态互斥访问 |
| `Arc<RwLock>` | 读写分离，多读单写 |
| `AtomicU64` | 无锁原子计数器 |
| `mpsc` channel | 多生产者单消费者 |
| `oneshot` channel | 请求-响应模式 |
| `broadcast` channel | 广播订阅 |
| `Semaphore` | 并发限流（控制最大并发数） |
| `Rayon` | CPU 密集型并行计算(par_iter) |
| 并发爬虫 | Semaphore + spawn 实战组合 |

### integrated-service — 整合微服务

整合 HTTP + MySQL + Redis + Kafka 的完整用户管理服务：
- **写入**: MySQL 持久化 → Redis 缓存回填 → Kafka 事件通知
- **读取**: Redis 缓存优先 → 缓存未命中查 MySQL → 回填缓存 (Cache Aside)
- **删除**: MySQL 删除 → 清除 Redis 缓存 → Kafka 事件通知
- **健康检查**: `/health` 返回各组件连接状态
- **Kafka 可选**: 未连接时自动降级，不影响核心功能

## 测试

```bash
# 无需外部服务（50 个测试中的 46 个）
cargo test -p shared -p http-api -p concurrent-tasks
cargo test -p kafka-messaging -- tests::test_order_event

# 需要 docker-compose up -d
cargo test -p mysql-crud
cargo test -p redis-cache
cargo test -p kafka-messaging
cargo test -p integrated-service

# 全部
cargo test
```

需要外部服务的测试在服务不可用时**自动跳过**，不会报错。

## 技术栈

| 分类 | 依赖 | 用途 |
|------|------|------|
| 异步运行时 | tokio 1.40 | async/await 基础设施 |
| HTTP 框架 | axum 0.8 | RESTful API |
| 中间件 | tower / tower-http | Trace、CORS |
| 数据库 | sqlx 0.8 | MySQL 异步驱动 |
| 缓存 | redis 0.26 | Redis 异步客户端 |
| 消息队列 | rdkafka 0.36 | Kafka 生产/消费 |
| 并行计算 | rayon 1.10 | CPU 密集型任务 |
| 序列化 | serde / serde_json | JSON 处理 |
| 错误处理 | thiserror / anyhow | 类型化错误 |
| 日志 | tracing / tracing-subscriber | 结构化日志 |

## 学习路径建议

```
基础语法 (src/)
    ↓
http-api          → 掌握 Axum + 异步 Web 开发
    ↓
mysql-crud        → 掌握 SQLx + 数据库操作
    ↓
redis-cache       → 掌握缓存策略 + 分布式锁
    ↓
kafka-messaging   → 掌握消息驱动架构
    ↓
concurrent-tasks  → 深入理解 Rust 并发模型
    ↓
integrated-service → 综合实战，整合全部技术栈
```

## 环境要求

- Rust 1.85+ (edition 2024)
- Docker & Docker Compose（运行 MySQL/Redis/Kafka）
