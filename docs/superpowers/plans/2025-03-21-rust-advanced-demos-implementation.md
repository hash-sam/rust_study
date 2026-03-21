# Rust Advanced Demos 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为 rust_study 项目添加 6 个生产级实际应用 demo，包括 HTTP 服务、MySQL、Redis、Kafka、并发处理和集成服务

**Architecture:** 使用 Cargo workspace 组织，每个 demo 是独立的 crate，共享依赖版本和通用模块。基于 Tokio 异步运行时，使用 Axum (HTTP)、SQLx (MySQL)、redis-rs (Redis)、rust-rdkafka (Kafka) 等业界标准库。

**Tech Stack:** Tokio 1.40, Axum 0.8, SQLx 0.8, redis 0.26, rdkafka 0.36, serde, tracing, anyhow, thiserror

---

## 文件结构概览

```
rust_study/
├── Cargo.toml                           # Workspace 配置
├── Cargo.lock                           # Lock 文件
├── src/                                 # 现有基础内容（保持不变）
│   ├── main.rs
│   ├── basics/
│   ├── control_flow/
│   └── ...
├── demos/                               # 新的深入 demo 目录
│   ├── shared/                          # 共享模块
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── error.rs                 # 统一错误处理
│   │       ├── config.rs                # 配置管理
│   │       └── lib.rs
│   ├── http-api/                        # Demo 1
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── mysql-crud/                      # Demo 2
│   │   ├── Cargo.toml
│   │   ├── migrations/
│   │   └── src/
│   ├── redis-cache/                     # Demo 3
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── kafka-messaging/                 # Demo 4
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── concurrent-tasks/                # Demo 5
│   │   ├── Cargo.toml
│   │   └── src/
│   └── integrated-service/              # Demo 6
│       ├── Cargo.toml
│       ├── docker-compose.yml
│       └── src/
└── docs/
    └── superpowers/
        ├── specs/
        │   └── 2025-03-21-rust-advanced-demos-design.md
        └── plans/
            └── 2025-03-21-rust-advanced-demos-implementation.md
```

---

## 第一阶段：基础设施搭建

### Task 1: 配置 Cargo Workspace

**Files:**
- Modify: `Cargo.toml` (root)
- Create: `demos/shared/Cargo.toml`
- Create: `demos/shared/src/lib.rs`

- [ ] **Step 1: 修改根 Cargo.toml 为 workspace 配置**

```toml
[workspace]
members = [
    "src",
    "demos/shared",
    "demos/http-api",
    "demos/mysql-crud",
    "demos/redis-cache",
    "demos/kafka-messaging",
    "demos/concurrent-tasks",
    "demos/integrated-service",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["rust_study"]

[workspace.dependencies]
# 异步运行时
tokio = { version = "1.40", features = ["full"] }
# HTTP 框架
axum = { version = "0.8", features = ["macros"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["trace", "cors", "limit"] }
# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# 数据库
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql", "chrono", "uuid"] }
# Redis
redis = { version = "0.26", features = ["tokio-comp", "connection-manager", "aio"] }
# Kafka
rdkafka = { version = "0.36", features = ["tokio"] }
# 异步工具
async-trait = "0.1"
futures = "0.3"
# 并行计算
rayon = "1.10"
# 随机数
rand = "0.8"
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
axum-test = "16"
```

- [ ] **Step 2: 修改 src/Cargo.toml**

在 `src/Cargo.toml` 末尾添加：
```toml
[package]
name = "rust_study"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- [ ] **Step 3: 创建 demos/shared/Cargo.toml**

```toml
[package]
name = "shared"
version.workspace = true
edition.workspace = true

[dependencies]
# 工作区依赖
anyhow.workspace = true
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
config.workspace = true
chrono.workspace = true
uuid.workspace = true
tokio.workspace = true
sqlx.workspace = true
redis.workspace = true
```

- [ ] **Step 4: 创建 demos/shared/src/lib.rs**

```rust
//! 共享模块 - 错误处理、配置管理、日志等

pub mod error;
pub mod config;

pub use error::{AppError, AppResult};
pub use config::AppConfig;
```

- [ ] **Step 5: 验证编译**

```bash
cargo check --workspace
```

Expected: 编译成功，可能有警告（因为没有实现模块）

- [ ] **Step 6: 提交**

```bash
git add Cargo.toml src/Cargo.toml demos/shared/Cargo.toml demos/shared/src/lib.rs
git commit -m "feat: setup cargo workspace structure"
```

---

### Task 2: 实现统一错误处理

**Files:**
- Create: `demos/shared/src/error.rs`

- [ ] **Step 1: 创建错误处理模块**

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json};
use serde_json::json;
use std::fmt;

/// 应用统一错误类型
#[derive(Debug)]
pub enum AppError {
    /// 数据库错误
    Database(String),
    /// 缓存错误
    Cache(String),
    /// Kafka 错误
    Messaging(String),
    /// 验证错误
    Validation(String),
    /// 未找到
    NotFound(String),
    /// 内部错误
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Cache(msg) => write!(f, "Cache error: {}", msg),
            AppError::Messaging(msg) => write!(f, "Messaging error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// 类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 从 anyhow::Error 转换
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// Axum 响应实现
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            AppError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, "validation_error", msg)
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, "not_found", msg)
            }
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "database_error", "服务暂时不可用".to_string())
            }
            AppError::Cache(msg) => {
                tracing::warn!("Cache error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "cache_error", "缓存服务异常".to_string())
            }
            AppError::Messaging(msg) => {
                tracing::error!("Messaging error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "messaging_error", "消息服务异常".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "内部服务错误".to_string())
            }
        };

        let body = json!({
            "error": error_type,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        (status, Json(body)).into_response()
    }
}

/// 从 sqlx::Error 转换
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

/// 从 redis::RedisError 转换
impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::Cache(err.to_string())
    }
}
```

- [ ] **Step 2: 更新 shared/src/lib.rs**

```rust
//! 共享模块 - 错误处理、配置管理、日志等

pub mod error;

pub use error::{AppError, AppResult};
```

- [ ] **Step 3: 验证编译**

```bash
cargo check -p shared
```

Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add demos/shared/src/error.rs demos/shared/src/lib.rs
git commit -m "feat: implement unified error handling"
```

---

### Task 3: 实现配置管理

**Files:**
- Create: `demos/shared/src/config.rs`
- Create: `demos/shared/config.example.yml`

- [ ] **Step 1: 创建配置管理模块**

```rust
use config::{Config, Environment, File};
use serde::Deserialize;
use std::path::Path;

/// 应用配置
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: Option<DatabaseConfig>,
    pub redis: Option<RedisConfig>,
    pub kafka: Option<KafkaConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
}

impl AppConfig {
    /// 从文件和环境变量加载配置
    ///
    /// # Arguments
    /// * `path` - 配置文件路径，如果为 None 则使用默认路径
    ///
    /// # Errors
    /// 如果配置文件不存在或格式错误，返回错误
    pub fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let config_path = path.unwrap_or("config.yml");

        let mut settings = Config::builder();

        // 加载配置文件
        if Path::new(config_path).exists() {
            settings = settings.add_source(File::with_name(config_path));
        }

        // 添加环境变量覆盖（前缀 APP_）
        // 例如: APP_SERVER__PORT=8080
        settings = settings.add_source(
            Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        );

        let settings = settings.build()?;

        let config: AppConfig = settings.try_deserialize()?;

        Ok(config)
    }

    /// 创建测试配置
    pub fn test_config() -> Self {
        AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: None,
            redis: None,
            kafka: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_config() {
        let config = AppConfig::test_config();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3000);
    }
}
```

- [ ] **Step 2: 更新 shared/src/lib.rs**

```rust
//! 共享模块 - 错误处理、配置管理、日志等

pub mod error;
pub mod config;

pub use error::{AppError, AppResult};
pub use config::AppConfig;
```

- [ ] **Step 3: 创建示例配置文件**

```yaml
# 配置文件示例
# 复制此文件为 config.yml 并修改相应配置

server:
  host: "127.0.0.1"
  port: 3000

# database:
#   url: "mysql://root:password@localhost:3306/rust_study"
#   max_connections: 10

# redis:
#   url: "redis://127.0.0.1:6379"
#   max_connections: 10

# kafka:
#   bootstrap_servers: "localhost:9092"
#   group_id: "rust_study_group"
```

- [ ] **Step 4: 验证编译和测试**

```bash
cargo check -p shared
cargo test -p shared
```

Expected: 编译成功，测试通过

- [ ] **Step 5: 提交**

```bash
git add demos/shared/src/config.rs demos/shared/config.example.yml demos/shared/src/lib.rs
git commit -m "feat: implement configuration management"
```

---

## 第二阶段：核心 Demo 实现

### Task 4: 创建 http-api 基础结构

**Files:**
- Create: `demos/http-api/Cargo.toml`
- Create: `demos/http-api/src/main.rs`
- Create: `demos/http-api/config.yml`
- Create: `demos/http-api/README.md`

- [ ] **Step 1: 创建 http-api/Cargo.toml**

```toml
[package]
name = "http-api"
version.workspace = true
edition.workspace = true

[dependencies]
# 共享模块
shared = { path = "../shared" }

# 工作区依赖
tokio.workspace = true
axum.workspace = true
tower.workspace = true
tower-http.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
```

- [ ] **Step 2: 创建基础 main.rs**

```rust
use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::{json, Value};
use shared::{AppConfig, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    tracing::info!("Starting HTTP API server on {}:{}", config.server.host, config.server.port);

    // 构建路由
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check));

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 健康检查端点
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "http-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

- [ ] **Step 3: 创建配置文件**

```yaml
server:
  host: "127.0.0.1"
  port: 3000
```

- [ ] **Step 4: 创建 README.md**

```markdown
# HTTP API Demo

这是一个使用 Axum 框架构建的 RESTful API 服务示例。

## 功能

- 健康检查端点
- 请求日志中间件
- 统一错误处理

## 运行

```bash
cd demos/http-api
cargo run
```

## 测试

```bash
curl http://127.0.0.1:3000/health
```
```

- [ ] **Step 5: 验证运行**

```bash
cargo run -p http-api
```

在另一个终端测试：
```bash
curl http://127.0.0.1:3000/health
```

Expected: 返回 JSON 响应

- [ ] **Step 6: 提交**

```bash
git add demos/http-api/
git commit -m "feat: create http-api base structure with health check"
```

---

### Task 5: 实现 http-api 用户模型和路由

**Files:**
- Create: `demos/http-api/src/models.rs`
- Create: `demos/http-api/src/routes/mod.rs`
- Create: `demos/http-api/src/routes/users.rs`
- Create: `demos/http-api/src/state.rs`
- Modify: `demos/http-api/src/main.rs`

- [ ] **Step 0: 创建目录结构**

```bash
mkdir -p demos/http-api/src/routes
```

- [ ] **Step 1: 创建用户模型**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}
```

- [ ] **Step 2: 创建应用状态**

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::User;
use std::collections::HashMap;
use uuid::Uuid;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 3: 创建用户路由**

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::state::AppState;
use shared::{AppError, AppResult};

/// 获取所有用户
pub async fn get_users(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<User>>> {
    let users = state.users.read().await;
    let user_list: Vec<User> = users.values().cloned().collect();
    Ok(Json(user_list))
}

/// 获取单个用户
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let users = state.users.read().await;
    let user = users.get(&id)
        .ok_or_else(|| AppError::NotFound(format!("User {}", id)))?;
    Ok(Json(user.clone()))
}

/// 创建用户
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    // 验证
    if req.username.is_empty() {
        return Err(AppError::Validation("用户名不能为空".to_string()));
    }

    let id = Uuid::new_v4();
    let user = User {
        id,
        username: req.username,
        email: req.email,
        created_at: chrono::Utc::now(),
    };

    let mut users = state.users.write().await;
    users.insert(id, user.clone());

    Ok(Json(user))
}

/// 更新用户
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let mut users = state.users.write().await;
    let user = users.get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("User {}", id)))?;

    if let Some(username) = req.username {
        if username.is_empty() {
            return Err(AppError::Validation("用户名不能为空".to_string()));
        }
        user.username = username;
    }

    if let Some(email) = req.email {
        user.email = email;
    }

    Ok(Json(user.clone()))
}

/// 删除用户
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let mut users = state.users.write().await;
    users.remove(&id)
        .ok_or_else(|| AppError::NotFound(format!("User {}", id)))?;
    Ok(StatusCode::NO_CONTENT)
}
```

- [ ] **Step 4: 创建路由模块**

```rust
pub mod users;

use axum::Router;
use crate::state::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", users::routes())
}
```

- [ ] **Step 4a: 创建 routes/users.rs 文件**

该文件将在下一步创建，但需要先创建目录结构：

```bash
mkdir -p demos/http-api/src/routes
```
```

- [ ] **Step 5: 更新 routes/users.rs 添加路由定义**

在文件末尾添加：

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}
```

- [ ] **Step 6: 更新 main.rs**

```rust
use axum::{
    Router,
    response::Json,
};
use serde_json::{json, Value};
use shared::{AppConfig, AppResult};
use state::AppState;
use routes::create_routes;

mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    tracing::info!("Starting HTTP API server on {}:{}", config.server.host, config.server.port);

    // 创建应用状态
    let state = AppState::new();

    // 构建路由
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .merge(create_routes())
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 健康检查端点
async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "http-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

- [ ] **Step 7: 验证编译**

```bash
cargo check -p http-api
```

- [ ] **Step 8: 提交**

```bash
git add demos/http-api/src/
git commit -m "feat: implement user CRUD operations in http-api"
```

---

### Task 6: 为 http-api 添加集成测试

**Files:**
- Create: `demos/http-api/tests/users_api_test.rs`

- [ ] **Step 1: 添加测试依赖到 Cargo.toml**

```toml
[dev-dependencies]
tokio-test.workspace = true
axum-test = "16"
```

同时添加 lib.rs 支持：

- [ ] **Step 2: 创建 lib.rs**

```rust
//! HTTP API library

pub mod models;
pub mod routes;
pub mod state;
pub use state::AppState;
```

- [ ] **Step 3: 更新 main.rs 使用 lib**

在 main.rs 顶部添加：

```rust
// 库根
http_api::models;
http_api::routes;
http_api::state;
```

并移除 `mod models;` 等声明。

- [ ] **Step 4: 创建 lib.rs 的 health_check 函数**

```rust
pub async fn health_check() -> axum::response::Json<serde_json::Value> {
    use serde_json::json;
    Json(json!({
        "status": "ok",
        "service": "http-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

- [ ] **Step 5: 更新 main.rs**

```rust
use axum::{Router, response::Json};
use serde_json::{json, Value};
use shared::{AppConfig, AppResult};
use http_api::{routes::create_routes, state::AppState};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    tracing::info!("Starting HTTP API server on {}:{}", config.server.host, config.server.port);

    // 创建应用状态
    let state = AppState::new();

    // 构建路由
    let app = Router::new()
        .route("/", get(health_check_main))
        .route("/health", get(health_check_main))
        .merge(create_routes())
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 健康检查端点
async fn health_check_main() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "http-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

- [ ] **Step 6: 创建集成测试**

```rust
use axum::{
    body::Body,
    http::{Method, StatusCode},
};
use axum_test::TestServer;
use http_api::{models::User, routes::create_routes, state::AppState};
use serde_json::json;

#[tokio::test]
async fn test_health_check() {
    use http_api::health_check;
    let response = health_check().await;
    assert_eq!(response.0.get("status").unwrap(), "ok");
}

#[tokio::test]
async fn test_create_user() {
    let state = AppState::new();
    let app = create_routes().with_state(state);

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/users")
        .json(&json!({
            "username": "testuser",
            "email": "test@example.com"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let user: User = response.json();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let state = AppState::new();
    let app = create_routes().with_state(state);

    let server = TestServer::new(app).unwrap();

    let response = server
        .get(&format!("/users/{}", uuid::Uuid::new_v4()))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
```

- [ ] **Step 7: 运行测试**

```bash
cargo test -p http-api
```

Expected: 所有测试通过

- [ ] **Step 8: 提交**

```bash
git add demos/http-api/ src/lib.rs
git commit -m "test: add integration tests for http-api"
```

---

### Task 7: 创建 mysql-crud 基础结构

**Files:**
- Create: `demos/mysql-crud/Cargo.toml`
- Create: `demos/mysql-crud/src/main.rs`
- Create: `demos/mysql-crud/config.yml`
- Create: `demos/mysql-crud/migrations/001_create_users.sql`
- Create: `demos/mysql-crud/migrations/002_create_posts.sql`
- Create: `demos/mysql-crud/README.md`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "mysql-crud"
version.workspace = true
edition.workspace = true

[dependencies]
# 共享模块
shared = { path = "../shared" }

# 工作区依赖
tokio.workspace = true
sqlx.workspace = true
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

- [ ] **Step 2: 创建数据库迁移文件**

```sql
-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id BINARY(16) PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_username (username),
    INDEX idx_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

```sql
-- 创建帖子表
CREATE TABLE IF NOT EXISTS posts (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id),
    INDEX idx_created_at (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

- [ ] **Step 3: 创建 main.rs（基础版本）**

```rust
use sqlx::{MySql, Pool};
use shared::{AppConfig, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    let db_config = config.database
        .expect("Database configuration required");

    // 创建连接池
    let pool = create_pool(&db_config.url, db_config.max_connections).await?;
    tracing::info!("Connected to MySQL database");

    // 运行迁移
    run_migrations(&pool).await?;
    tracing::info!("Migrations completed");

    // TODO: 实现 CRUD 操作示例
    println!("MySQL CRUD demo - migrations completed successfully");

    Ok(())
}

/// 创建数据库连接池
async fn create_pool(url: &str, max_connections: u32) -> AppResult<Pool<MySql>> {
    let pool = sqlx::MySqlPool::connect_with(
        sqlx::mysql::MySqlConnectOptions::from_str(url)?
            .max_connections(max_connections as u32)
    ).await?;

    Ok(pool)
}

/// 运行数据库迁移
async fn run_migrations(pool: &Pool<MySql>) -> AppResult<()> {
    // 读取迁移文件
    let create_users = tokio::fs::read_to_string("migrations/001_create_users.sql").await?;
    let create_posts = tokio::fs::read_to_string("migrations/002_create_posts.sql").await?;

    // 执行迁移
    sqlx::query(&create_users).execute(pool).await?;
    sqlx::query(&create_posts).execute(pool).await?;

    Ok(())
}
```

- [ ] **Step 4: 创建配置文件**

```yaml
server:
  host: "127.0.0.1"
  port: 3001

database:
  url: "mysql://root:password@localhost:3306/rust_study"
  max_connections: 10
```

- [ ] **Step 5: 创建 README.md**

```markdown
# MySQL CRUD Demo

这是一个使用 SQLx 进行 MySQL 数据库操作的示例。

## 功能

- 数据库连接池管理
- 数据库迁移
- CRUD 操作
- 事务处理

## 前置条件

1. 安装 MySQL
2. 创建数据库:

```bash
mysql -u root -p
CREATE DATABASE rust_study;
```

## 运行

```bash
cd demos/mysql-crud
cargo run
```

## 运行迁移

迁移会在首次运行时自动执行。
```

- [ ] **Step 6: 验证编译**

```bash
cargo check -p mysql-crud
```

- [ ] **Step 7: 提交**

```bash
git add demos/mysql-crud/
git commit -m "feat: create mysql-crud base structure with migrations"
```

---

### Task 8: 实现 mysql-crud 模型和仓库

**Files:**
- Create: `demos/mysql-crud/src/models.rs`
- Create: `demos/mysql-crud/src/repository/mod.rs`
- Create: `demos/mysql-crud/src/repository/user_repository.rs`
- Modify: `demos/mysql-crud/src/main.rs`

**Note**: main.rs 需要声明 `mod models;` 和 `mod repository;` 以引入这些模块。

- [ ] **Step 1: 创建数据模型**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建用户
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

/// 更新用户
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// 帖子模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建帖子
#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub user_id: Uuid,
    pub title: String,
    pub content: Option<String>,
}

/// 用户详情（包含帖子）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithPosts {
    pub user: User,
    pub posts: Vec<Post>,
}
```

- [ ] **Step 2: 创建用户仓库**

```rust
use sqlx::{MySql, Pool};
use uuid::Uuid;
use crate::models::{User, CreateUser, UpdateUser, Post, CreatePost, UserWithPosts};
use shared::AppError;

pub struct UserRepository {
    pool: Pool<MySql>,
}

impl UserRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }

    /// 创建用户
    pub async fn create(&self, input: CreateUser) -> Result<User, AppError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, username, email, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(&input.username)
        .bind(&input.email)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to create user: {}", e)))?;

        // 获取刚创建的用户
        self.find_by_id(id).await
    }

    /// 根据 ID 查找用户
    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to find user: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("User {}", id)))
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to find user: {}", e)))
    }

    /// 获取所有用户
    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to find users: {}", e)))
    }

    /// 更新用户
    pub async fn update(&self, id: Uuid, input: UpdateUser) -> Result<User, AppError> {
        let mut query = String::from("UPDATE users SET updated_at = ?");
        let mut params = vec![];

        if let Some(username) = &input.username {
            query.push_str(", username = ?");
            params.push(username.clone());
        }

        if let Some(email) = &input.email {
            query.push_str(", email = ?");
            params.push(email.clone());
        }

        query.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&query);
        q = q.bind(chrono::Utc::now());

        for param in params {
            q = q.bind(param);
        }

        q = q.bind(id);

        q.execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update user: {}", e)))?;

        self.find_by_id(id).await
    }

    /// 删除用户
    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete user: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User {}", id)));
        }

        Ok(())
    }

    /// 获取用户及其帖子
    pub async fn find_with_posts(&self, id: Uuid) -> Result<UserWithPosts, AppError> {
        let user = self.find_by_id(id).await?;

        let posts = sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to find posts: {}", e)))?;

        Ok(UserWithPosts { user, posts })
    }

    /// 创建帖子
    pub async fn create_post(&self, input: CreatePost) -> Result<Post, AppError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO posts (id, user_id, title, content, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(input.user_id)
        .bind(&input.title)
        .bind(&input.content)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(format!("Failed to create post: {}", e)))?;

        self.find_post_by_id(id).await
    }

    /// 根据 ID 查找帖子
    pub async fn find_post_by_id(&self, id: Uuid) -> Result<Post, AppError> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(format!("Failed to find post: {}", e)))?
            .ok_or_else(|| AppError::NotFound(format!("Post {}", id)))
    }
}
```

- [ ] **Step 3: 创建仓库模块**

首先创建目录：
```bash
mkdir -p demos/mysql-crud/src/repository
```

然后创建 `demos/mysql-crud/src/repository/mod.rs`:

```rust
pub mod user_repository;

pub use user_repository::UserRepository;
```

- [ ] **Step 4: 更新 main.rs 添加演示**

```rust
// 声明模块
mod models;
mod repository;

use sqlx::{MySql, Pool};
use shared::{AppConfig, AppResult};
use repository::UserRepository;
use models::{CreateUser, CreatePost};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    let db_config = config.database
        .expect("Database configuration required");

    // 创建连接池
    let pool = create_pool(&db_config.url, db_config.max_connections).await?;
    tracing::info!("Connected to MySQL database");

    // 运行迁移
    run_migrations(&pool).await?;
    tracing::info!("Migrations completed");

    // 创建仓库
    let user_repo = UserRepository::new(pool);

    // 演示 CRUD 操作
    demonstrate_crud(&user_repo).await?;

    Ok(())
}

/// 演示 CRUD 操作
async fn demonstrate_crud(repo: &UserRepository) -> AppResult<()> {
    println!("\n=== MySQL CRUD 演示 ===\n");

    // 创建用户
    println!("1. 创建用户");
    let user = repo.create(CreateUser {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
    }).await?;
    println!("   创建用户: {} ({})", user.username, user.id);

    let user2 = repo.create(CreateUser {
        username: "bob".to_string(),
        email: "bob@example.com".to_string(),
    }).await?;
    println!("   创建用户: {} ({})", user2.username, user2.id);

    // 查找用户
    println!("\n2. 查找用户");
    let found = repo.find_by_id(user.id).await?;
    println!("   找到用户: {} <{}>", found.username, found.email);

    // 列出所有用户
    println!("\n3. 列出所有用户");
    let users = repo.find_all().await?;
    println!("   总共 {} 个用户:", users.len());
    for u in &users {
        println!("   - {} <{}>", u.username, u.email);
    }

    // 更新用户
    println!("\n4. 更新用户");
    let updated = repo.update(user.id, models::UpdateUser {
        email: Some("alice.new@example.com".to_string()),
        username: None,
    }).await?;
    println!("   更新后: {} <{}>", updated.username, updated.email);

    // 创建帖子
    println!("\n5. 创建帖子");
    let post = repo.create_post(CreatePost {
        user_id: user.id,
        title: "我的第一篇帖子".to_string(),
        content: Some("这是帖子的内容".to_string()),
    }).await?;
    println!("   创建帖子: {}", post.title);

    // 获取用户及其帖子
    println!("\n6. 获取用户及其帖子");
    let user_with_posts = repo.find_with_posts(user.id).await?;
    println!("   用户: {}", user_with_posts.user.username);
    println!("   帖子数: {}", user_with_posts.posts.len());
    for post in &user_with_posts.posts {
        println!("   - {}", post.title);
    }

    // 删除用户
    println!("\n7. 删除用户");
    repo.delete(user2.id).await?;
    println!("   删除用户: {}", user2.username);

    let users = repo.find_all().await?;
    println!("   剩余 {} 个用户", users.len());

    Ok(())
}

/// 创建数据库连接池
async fn create_pool(url: &str, max_connections: u32) -> AppResult<Pool<MySql>> {
    let pool = sqlx::MySqlPool::connect_with(
        sqlx::mysql::MySqlConnectOptions::from_str(url)?
            .max_connections(max_connections as u32)
    ).await?;

    Ok(pool)
}

/// 运行数据库迁移
async fn run_migrations(pool: &Pool<MySql>) -> AppResult<()> {
    let create_users = tokio::fs::read_to_string("migrations/001_create_users.sql").await?;
    let create_posts = tokio::fs::read_to_string("migrations/002_create_posts.sql").await?;

    sqlx::query(&create_users).execute(pool).await?;
    sqlx::query(&create_posts).execute(pool).await?;

    Ok(())
}
```

- [ ] **Step 5: 验证编译**

```bash
cargo check -p mysql-crud
```

- [ ] **Step 6: 提交**

```bash
git add demos/mysql-crud/src/
git commit -m "feat: implement user repository with CRUD operations"
```

---

### Task 9: 创建 redis-cache 基础结构

**Files:**
- Create: `demos/redis-cache/Cargo.toml`
- Create: `demos/redis-cache/src/main.rs`
- Create: `demos/redis-cache/config.yml`
- Create: `demos/redis-cache/README.md`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "redis-cache"
version.workspace = true
edition.workspace = true

[dependencies]
# 共享模块
shared = { path = "../shared" }

# 工作区依赖
tokio.workspace = true
redis.workspace = true
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
chrono.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
```

- [ ] **Step 2: 创建基础 main.rs**

```rust
use redis::{Client, ConnectionManager};
use shared::{AppConfig, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // 加载配置
    let config = AppConfig::load(None)?;
    let redis_config = config.redis
        .expect("Redis configuration required");

    // 创建 Redis 客户端
    let client = Client::open(redis_config.url.clone())?;
    let manager = ConnectionManager::new(client).await?;
    tracing::info!("Connected to Redis at {}", redis_config.url);

    // TODO: 实现缓存示例
    println!("Redis Cache demo - connected successfully");

    Ok(())
}
```

- [ ] **Step 3: 创建配置文件**

```yaml
server:
  host: "127.0.0.1"
  port: 3002

redis:
  url: "redis://127.0.0.1:6379"
  max_connections: 10
```

- [ ] **Step 4: 创建 README**

```markdown
# Redis Cache Demo

这是一个使用 Redis 实现缓存服务的示例。

## 功能

- Redis 连接管理
- 通用缓存接口
- 缓存策略（Cache Aside、Write Through）
- 缓存防护（穿透、雪崩、击穿）

## 前置条件

安装并启动 Redis:

```bash
# macOS
brew install redis
brew services start redis

# Linux
sudo systemctl start redis
```

## 运行

```bash
cd demos/redis-cache
cargo run
```
```

- [ ] **Step 5: 验证编译**

```bash
cargo check -p redis-cache
```

- [ ] **Step 6: 提交**

```bash
git add demos/redis-cache/
git commit -m "feat: create redis-cache base structure"
```

---

### Task 10: 实现 Redis 缓存接口和策略

**Files:**
- Create: `demos/redis-cache/src/cache.rs`
- Create: `demos/redis-cache/src/strategies.rs`
- Create: `demos/redis-cache/src/protection.rs`
- Modify: `demos/redis-cache/src/main.rs`

- [ ] **Step 1: 创建缓存接口**

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use shared::AppError;

/// 缓存接口
#[async_trait]
pub trait Cache: Send + Sync {
    /// 获取缓存
    async fn get<T>(&self, key: &str) -> Result<Option<T>, AppError>
    where
        T: for<'de> Deserialize<'de>;

    /// 设置缓存
    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), AppError>
    where
        T: Serialize;

    /// 删除缓存
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// 检查键是否存在
    async fn exists(&self, key: &str) -> Result<bool, AppError>;

    /// 设置过期时间
    async fn expire(&self, key: &str, ttl: Duration) -> Result<(), AppError>;
}
```

- [ ] **Step 2: 创建 Redis 缓存实现**

```rust
use super::Cache;
use redis::{ConnectionManager, AsyncCommands};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use shared::AppError;

pub struct RedisCache {
    manager: ConnectionManager,
}

impl RedisCache {
    pub fn new(manager: ConnectionManager) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, AppError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.manager.clone();
        let value: Option<String> = conn.get(key).await
            .map_err(|e| AppError::Cache(format!("Failed to get cache: {}", e)))?;

        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)
                    .map_err(|e| AppError::Cache(format!("Failed to deserialize: {}", e)))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), AppError>
    where
        T: Serialize,
    {
        let mut conn = self.manager.clone();
        let serialized = serde_json::to_string(value)
            .map_err(|e| AppError::Cache(format!("Failed to serialize: {}", e)))?;

        conn.set_ex(key, serialized, ttl.as_secs() as usize).await
            .map_err(|e| AppError::Cache(format!("Failed to set cache: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.manager.clone();
        conn.del(key).await
            .map_err(|e| AppError::Cache(format!("Failed to delete cache: {}", e)))?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, AppError> {
        let mut conn = self.manager.clone();
        let exists: bool = conn.exists(key).await
            .map_err(|e| AppError::Cache(format!("Failed to check exists: {}", e)))?;
        Ok(exists)
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<(), AppError> {
        let mut conn = self.manager.clone();
        conn.expire(key, ttl.as_secs() as usize).await
            .map_err(|e| AppError::Cache(format!("Failed to set expire: {}", e)))?;
        Ok(())
    }
}
```

- [ ] **Step 3: 创建缓存策略**

```rust
use super::Cache;
use std::time::Duration;
use shared::AppError;

/// Cache Aside 策略
///
/// 1. 先查缓存
/// 2. 缓存不存在，查数据库
/// 3. 将数据库结果写入缓存
pub async fn cache_aside<C, F, T>(
    cache: &C,
    key: &str,
    loader: F,
    ttl: Duration,
) -> Result<T, AppError>
where
    C: Cache,
    F: FnOnce() -> Result<T, AppError>,
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    // 先查缓存
    if let Some(value) = cache.get::<T>(key).await? {
        tracing::debug!("Cache hit: {}", key);
        return Ok(value);
    }

    tracing::debug!("Cache miss: {}", key);

    // 缓存未命中，加载数据
    let value = loader()?;

    // 写入缓存
    cache.set(key, &value, ttl).await?;

    Ok(value)
}

/// Write Through 策略
///
/// 1. 先写数据库
/// 2. 数据库写入成功后，写缓存
pub async fn write_through<C, F, T>(
    cache: &C,
    key: &str,
    value: &T,
    db_writer: F,
    ttl: Duration,
) -> Result<T, AppError>
where
    C: Cache,
    F: FnOnce(&T) -> Result<T, AppError>,
    T: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone,
{
    // 先写数据库
    let result = db_writer(value)?;

    // 数据库写入成功，写缓存
    cache.set(key, &result, ttl).await?;

    Ok(result)
}
```

- [ ] **Step 4: 创建缓存保护**

```rust
use super::Cache;
use std::time::Duration;
use shared::AppError;

/// 防止缓存穿透
///
/// 对于不存在的数据，缓存一个空值
pub async fn protect_penetration<C, F, T>(
    cache: &C,
    key: &str,
    loader: F,
    ttl: Duration,
    null_ttl: Duration,
) -> Result<Option<T>, AppError>
where
    C: Cache,
    F: FnOnce() -> Result<Option<T>, AppError>,
    T: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone,
{
    // 尝试从缓存获取
    if let Ok(Some(value)) = cache.get::<T>(key).await {
        return Ok(Some(value));
    }

    // 检查是否是空值标记
    let null_key = format!("{}:null", key);
    if cache.exists(&null_key).await? {
        return Ok(None);
    }

    // 加载数据
    let value = loader()?;

    match value {
        Some(ref v) => {
            // 缓存实际值
            cache.set(key, v, ttl).await?;
        }
        None => {
            // 缓存空值标记，防止穿透
            cache.set(&null_key, &"null", null_ttl).await?;
        }
    }

    Ok(value)
}

/// 防止缓存雪崩
///
/// 为不同的 key 设置随机的过期时间
pub fn random_ttl(base_ttl: Duration, jitter: Duration) -> Duration {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_secs = rng.gen_range(0..=jitter.as_secs());
    base_ttl + Duration::from_secs(random_secs)
}
```

- [ ] **Step 5: 更新 main.rs 添加演示**

```rust
mod cache;
mod strategies;
mod protection;

use cache::{Cache, RedisCache};
use strategies::{cache_aside, write_through};
use protection::{protect_penetration, random_ttl};
use redis::{Client, ConnectionManager};
use shared::{AppConfig, AppResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt().with_target(false).init();

    let config = AppConfig::load(None)?;
    let redis_config = config.redis.expect("Redis configuration required");

    let client = Client::open(redis_config.url.clone())?;
    let manager = ConnectionManager::new(client).await?;
    tracing::info!("Connected to Redis");

    let cache = RedisCache::new(manager);

    demonstrate_cache(&cache).await?;

    Ok(())
}

async fn demonstrate_cache(cache: &RedisCache) -> AppResult<()> {
    println!("\n=== Redis 缓存演示 ===\n");

    // 基础操作
    println!("1. 基础缓存操作");
    let user = User {
        id: "1".to_string(),
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    cache.set("user:1", &user, Duration::from_secs(60)).await?;
    println!("   缓存用户: user:1");

    let cached: Option<User> = cache.get("user:1").await?;
    println!("   从缓存获取: {:?}", cached);

    let exists = cache.exists("user:1").await?;
    println!("   键存在: {}", exists);

    // Cache Aside 策略
    println!("\n2. Cache Aside 策略");
    let user = cache_aside(
        cache,
        "user:2",
        || {
            Ok(User {
                id: "2".to_string(),
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            })
        },
        Duration::from_secs(60),
    ).await?;
    println!("   加载用户: {}", user.name);

    // 第二次从缓存获取
    let user = cache_aside(
        cache,
        "user:2",
        || {
            println!("   从数据库加载（不应该看到这条）");
            Ok(User {
                id: "2".to_string(),
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            })
        },
        Duration::from_secs(60),
    ).await?;
    println!("   从缓存获取: {}", user.name);

    // 防穿透
    println!("\n3. 防止缓存穿透");
    let result = protect_penetration(
        cache,
        "user:999",
        || Ok::<_, AppError>(None),
        Duration::from_secs(60),
        Duration::from_secs(300),
    ).await?;
    println!("   查询不存在的用户: {:?}", result);

    // 第二次查询（从缓存获取空值）
    let result = protect_penetration(
        cache,
        "user:999",
        || {
            println!("   查询数据库（不应该看到这条）");
            Ok::<_, AppError>(None)
        },
        Duration::from_secs(60),
        Duration::from_secs(300),
    ).await?;
    println!("   从缓存获取: {:?}", result);

    // 随机 TTL（防雪崩）
    println!("\n4. 随机 TTL（防雪崩）");
    for i in 1..=5 {
        let ttl = random_ttl(Duration::from_secs(60), Duration::from_secs(30));
        println!("   Key {} TTL: {}s", i, ttl.as_secs());
    }

    Ok(())
}
```

- [ ] **Step 6: 更新 Cargo.toml 添加依赖**

```toml
async-trait = "0.1"
rand = "0.8"
```

- [ ] **Step 7: 验证编译**

```bash
cargo check -p redis-cache
```

- [ ] **Step 8: 提交**

```bash
git add demos/redis-cache/src/ demos/redis-cache/Cargo.toml
git commit -m "feat: implement cache strategies and protection"
```

---

### Task 11: 创建 kafka-messaging 基础结构

**Files:**
- Create: `demos/kafka-messaging/Cargo.toml`
- Create: `demos/kafka-messaging/src/main.rs`
- Create: `demos/kafka-messaging/config.yml`
- Create: `demos/kafka-messaging/README.md`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "kafka-messaging"
version.workspace = true
edition.workspace = true

[[bin]]
name = "producer"
path = "src/producer_main.rs"

[[bin]]
name = "consumer"
path = "src/consumer_main.rs"

[dependencies]
# 共享模块
shared = { path = "../shared" }

# 工作区依赖
tokio.workspace = true
rdkafka.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
```

- [ ] **Step 2: 创建消息模型**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub timestamp: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
}

impl Message {
    pub fn new(event_type: &str, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now().timestamp(),
            event_type: event_type.to_string(),
            payload,
        }
    }
}
```

- [ ] **Step 3: 创建 producer_main.rs**

```rust
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use std::time::Duration;
use shared::{AppConfig, AppResult};
use models::Message;

mod models;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt().with_target(false).init();

    let config = AppConfig::load(None)?;
    let kafka_config = config.kafka
        .expect("Kafka configuration required");

    // 创建 producer
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", &kafka_config.bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()?;

    tracing::info!("Kafka producer started");

    // 发送消息
    for i in 1..=10 {
        let message = Message::new("user.created", serde_json::json!({
            "user_id": Uuid::new_v4(),
            "username": format!("user{}", i),
            "email": format!("user{}@example.com", i),
        }));

        let payload = serde_json::to_vec(&message)?;
        let key = message.id.to_string();

        let record = FutureRecord::to("user-events")
            .key(&key)
            .payload(&payload);

        match producer.send(record, Timeout::After(Duration::from_secs(5))).await {
            Ok((partition, offset)) => {
                tracing::info!("Sent: partition={}, offset={}", partition, offset);
            }
            Err((err, _)) => {
                tracing::error!("Failed to send: {}", err);
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // Flush 确保所有消息发送完成
    producer.flush(Timeout::After(Duration::from_secs(30)))?;

    Ok(())
}
```

- [ ] **Step 4: 创建 consumer_main.rs**

```rust
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::Message as KafkaMessage,
};
use shared::{AppConfig, AppResult};
use models::Message;

mod models;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt().with_target(false).init();

    let config = AppConfig::load(None)?;
    let kafka_config = config.kafka
        .expect("Kafka configuration required");

    // 创建 consumer
    let consumer: StreamConsumer = rdkafka::config::ClientConfig::new()
        .set("group.id", &kafka_config.group_id)
        .set("bootstrap.servers", &kafka_config.bootstrap_servers)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()?;

    // 订阅主题
    consumer.subscribe(&["user-events"])?;

    tracing::info!("Kafka consumer started");

    // 消费消息
    let mut stream = consumer.stream();

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    match serde_json::from_slice::<Message>(payload) {
                        Ok(msg) => {
                            tracing::info!("Received: event_type={}, id={}",
                                msg.event_type, msg.id);
                        }
                        Err(e) => {
                            tracing::error!("Failed to deserialize: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("Kafka error: {}", e);
            }
        }
    }

    Ok(())
}
```

- [ ] **Step 5: 创建配置和 README**

配置文件:
```yaml
server:
  host: "127.0.0.1"
  port: 3003

kafka:
  bootstrap_servers: "localhost:9092"
  group_id: "rust_study_group"
```

README:
```markdown
# Kafka Messaging Demo

Kafka 消息队列示例。

## 前置条件

启动 Kafka 和 ZooKeeper:

```bash
# 使用 Docker
docker run -d --name zookeeper -p 2181:2181 zookeeper
docker run -d --name kafka -p 9092:9092 \
  --link zookeeper:zookeeper \
  -e KAFKA_ZOOKEEPER_CONNECT=zookeeper:2181 \
  -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 \
  confluentinc/cp-kafka

# 等待 Kafka 启动（约30秒）
sleep 30

# 创建 topic
docker exec -it kafka kafka-topics --create --topic user-events --bootstrap-server localhost:9092 --partitions 3 --replication-factor 1
```

或者使用自动创建 topic（需要在 Kafka 配置中启用 `auto.create.topics.enable=true`）。

## 运行

**Producer:**
```bash
cd demos/kafka-messaging
cargo run --bin producer
```

**Consumer:**
```bash
cd demos/kafka-messaging
cargo run --bin consumer
```
```

---

### Task 12: 创建 concurrent-tasks 基础结构

**Files:**
- Create: `demos/concurrent-tasks/Cargo.toml`
- Create: `demos/concurrent-tasks/src/main.rs`
- Create: `demos/concurrent-tasks/README.md`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "concurrent-tasks"
version.workspace = true
edition.workspace = true

[dependencies]
# 工作区依赖
tokio.workspace = true
rayon = "1.10"
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
```

- [ ] **Step 2: 创建 main.rs（交互式菜单）**

```rust
use std::io::{self, Write};

mod channels;
mod shared_state;
mod threadpools;
mod async_concurrency;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();

    loop {
        println!("\n=== Rust 并发编程演示 ===\n");
        println!("1. Channel 通信");
        println!("2. 共享状态");
        println!("3. 线程池");
        println!("4. 异步并发");
        println!("0. 退出");

        print!("\n选择: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => channels::demo().await?,
            "2" => shared_state::demo().await?,
            "3" => threadpools::demo().await?,
            "4" => async_concurrency::demo().await?,
            "0" => break,
            _ => println!("无效选择"),
        }
    }

    Ok(())
}
```

- [ ] **Step 3: 创建 README**

```markdown
# Concurrent Tasks Demo

Rust 并发编程示例。

## 功能

- Channel 通信 (mpsc, broadcast, watch, oneshot)
- 共享状态 (Arc<Mutex>, Arc<RwLock>, 原子类型)
- 线程池 (Tokio, Rayon)
- 异步并发 (join!, select!)

## 运行

```bash
cd demos/concurrent-tasks
cargo run
```
```

- [ ] **Step 4: 提交**

```bash
git add demos/concurrent-tasks/
git commit -m "feat: create concurrent-tasks base structure"
```

---

### Task 13: 实现 Channel 通信示例

**Files:**
- Create: `demos/concurrent-tasks/src/channels.rs`

**Note**: 此文件将在 src/ 目录下直接创建（channels.rs），不需要子目录。main.rs 中的 `mod channels;` 会自动声明此模块。

- [ ] **Step 1: 创建 Channel 示例**

```rust
use tokio::sync::{mpsc, broadcast, watch, oneshot};
use std::time::Duration;

pub async fn demo() -> anyhow::Result<()> {
    println!("\n=== Channel 通信演示 ===\n");

    mpsc_demo().await?;
    broadcast_demo().await?;
    watch_demo().await?;
    oneshot_demo().await?;

    Ok(())
}

/// mpsc: 多生产者单消费者
async fn mpsc_demo() -> anyhow::Result<()> {
    println!("1. mpsc (多生产者单消费者)");

    let (tx, mut rx) = mpsc::channel(100);

    // 启动生产者任务
    let producer1 = tokio::spawn({
        let tx = tx.clone();
        async move {
            for i in 1..=5 {
                tx.send(format!("Producer1: {}", i)).await.unwrap();
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    });

    let producer2 = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(format!("Producer2: {}", i)).await.unwrap();
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
    });

    // 消费者
    let consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(msg) = rx.recv().await {
            println!("   Received: {}", msg);
            count += 1;
            if count >= 10 {
                break;
            }
        }
    });

    producer1.await?;
    producer2.await?;
    consumer.await?;

    println!("   完成\n");
    Ok(())
}

/// broadcast: 广播通道
async fn broadcast_demo() -> anyhow::Result<()> {
    println!("2. broadcast (广播通道)");

    let (tx, _rx) = broadcast::channel(100);

    // 创建多个接收者
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    // 发送者
    let sender = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(format!("Message {}", i)).unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // 接收者 1
    let receiver1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("   Receiver1: {}", msg);
        }
    });

    // 接收者 2
    let receiver2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("   Receiver2: {}", msg);
        }
    });

    sender.await?;
    receiver1.await?;
    receiver2.await?;

    println!("   完成\n");
    Ok(())
}

/// watch: 多读者单写者
async fn watch_demo() -> anyhow::Result<()> {
    println!("3. watch (多读者单写者)");

    let (tx, mut rx1) = watch::channel(0);
    let mut rx2 = tx.subscribe();

    // 写者
    let writer = tokio::spawn(async move {
        for i in 1..=5 {
            println!("   Sending: {}", i);
            tx.send(i).unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // 读者 1
    let reader1 = tokio::spawn(async move {
        while rx1.changed().await.is_ok() {
            println!("   Reader1: {}", *rx1.borrow());
        }
    });

    // 读者 2
    let reader2 = tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("   Reader2: {}", *rx2.borrow());
        }
    });

    writer.await?;
    reader1.await?;
    reader2.await?;

    println!("   完成\n");
    Ok(())
}

/// oneshot: 一次性通道
async fn oneshot_demo() -> anyhow::Result<()> {
    println!("4. oneshot (一次性通道)");

    let (tx, rx) = oneshot::channel();

    // 发送者
    let sender = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        tx.send("Hello from oneshot!").unwrap();
    });

    // 接收者
    let receiver = tokio::spawn(async move {
        match rx.await {
            Ok(msg) => println!("   Received: {}", msg),
            Err(e) => println!("   Error: {}", e),
        }
    });

    sender.await?;
    receiver.await?;

    println!("   完成\n");
    Ok(())
}
```

- [ ] **Step 2: 提交**

```bash
git add demos/concurrent-tasks/src/channels.rs
git commit -m "feat: implement channel communication examples"
```

---

### Task 14: 实现共享状态示例

**Files:**
- Create: `demos/concurrent-tasks/src/shared_state.rs`

- [ ] **Step 1: 创建共享状态示例**

```rust
use tokio::sync::{Mutex, RwLock, Semaphore};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub async fn demo() -> anyhow::Result<()> {
    println!("\n=== 共享状态演示 ===\n");

    mutex_demo().await?;
    rwlock_demo().await?;
    atomic_demo().await?;
    semaphore_demo().await?;

    Ok(())
}

/// Arc<Mutex<T>>: 互斥锁
async fn mutex_demo() -> anyhow::Result<()> {
    println!("1. Arc<Mutex<T>> (互斥锁)");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut data = counter.lock().await;
            *data += 1;
            println!("   Task {} incremented to {}", i, *data);
        }));
    }

    for handle in handles {
        handle.await?;
    }

    println!("   Final value: {}", *counter.lock().await);
    println!("   完成\n");
    Ok(())
}

/// Arc<RwLock<T>>: 读写锁
async fn rwlock_demo() -> anyhow::Result<()> {
    println!("2. Arc<RwLock<T>> (读写锁)");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 读取者
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(tokio::spawn(async move {
            let r = data.read().await;
            println!("   Reader {} sees: {:?}", i, *r);
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("   Reader {} done", i);
        }));
    }

    // 写入者
    let data = Arc::clone(&data);
    handles.push(tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let mut w = data.write().await;
        w.push(4);
        println!("   Writer appended 4, now: {:?}", *w);
    }));

    for handle in handles {
        handle.await?;
    }

    println!("   Final data: {:?}", *data.read().await);
    println!("   完成\n");
    Ok(())
}

/// Arc<AtomicUsize>: 原子类型
async fn atomic_demo() -> anyhow::Result<()> {
    println!("3. Arc<AtomicUsize> (原子类型)");

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let old = counter.fetch_add(1, Ordering::SeqCst);
            println!("   Task {} incremented {} -> {}", i, old, old + 1);
        }));
    }

    for handle in handles {
        handle.await?;
    }

    println!("   Final value: {}", counter.load(Ordering::SeqCst));
    println!("   完成\n");
    Ok(())
}

/// Semaphore: 信号量
async fn semaphore_demo() -> anyhow::Result<()> {
    println!("4. Semaphore (信号量)");

    let semaphore = Arc::new(Semaphore::new(3)); // 最多3个并发
    let mut handles = vec![];

    for i in 0..6 {
        let semaphore = Arc::clone(&semaphore);
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("   Task {} started (active: {})",
                i,
                3 - semaphore.available_permits()
            );
            tokio::time::sleep(Duration::from_millis(200)).await;
            println!("   Task {} finished", i);
        }));
    }

    for handle in handles {
        handle.await?;
    }

    println!("   完成\n");
    Ok(())
}
```

- [ ] **Step 2: 提交**

```bash
git add demos/concurrent-tasks/src/shared_state.rs
git commit -m "feat: implement shared state examples"
```

---

### Task 15: 实现线程池和异步并发示例

**Files:**
- Create: `demos/concurrent-tasks/src/threadpools.rs`
- Create: `demos/concurrent-tasks/src/async_concurrency.rs`

- [ ] **Step 1: 创建线程池示例**

```rust
use std::time::Duration;
use rayon::prelude::*;

pub async fn demo() -> anyhow::Result<()> {
    println!("\n=== 线程池演示 ===\n");

    tokio_pool_demo().await?;
    rayon_demo()?;

    Ok(())
}

/// Tokio 线程池
async fn tokio_pool_demo() -> anyhow::Result<()> {
    println!("1. Tokio 线程池");

    let start = std::time::Instant::now();

    let mut handles = vec![];

    for i in 0..10 {
        handles.push(tokio::spawn(async move {
            // 模拟工作
            tokio::time::sleep(Duration::from_millis(100)).await;
            i * i
        }));
    }

    let results: Vec<usize> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    println!("   Results: {:?}", results);
    println!("   Time: {:?}", start.elapsed());
    println!("   完成\n");

    Ok(())
}

/// Rayon 并行计算
fn rayon_demo() -> anyhow::Result<()> {
    println!("2. Rayon 并行计算");

    let data: Vec<usize> = (1..=1_000_000).collect();

    let start = std::time::Instant::now();

    // 顺序计算
    let sum_sequential: usize = data.iter().sum();
    println!("   Sequential sum: {}", sum_sequential);

    // 并行计算
    let sum_parallel: usize = data.par_iter().sum();
    println!("   Parallel sum: {}", sum_parallel);

    println!("   Time: {:?}", start.elapsed());

    // 并行迭代器
    let squares: Vec<_> = (1..=10)
        .into_par_iter()
        .map(|x| x * x)
        .collect();
    println!("   Squares: {:?}", squares);

    println!("   完成\n");

    Ok(())
}
```

- [ ] **Step 2: 创建异步并发示例**

```rust
use tokio::time::{sleep, Duration, timeout};

pub async fn demo() -> anyhow::Result<()> {
    println!("\n=== 异步并发演示 ===\n");

    join_demo().await?;
    select_demo().await?;
    timeout_demo().await?;

    Ok(())
)

/// join!: 并行执行多个异步任务
async fn join_demo() -> anyhow::Result<()> {
    println!("1. join! (并行执行)");

    async fn task1() -> String {
        sleep(Duration::from_millis(100)).await;
        "Task 1 complete".to_string()
    }

    async fn task2() -> String {
        sleep(Duration::from_millis(200)).await;
        "Task 2 complete".to_string()
    }

    async fn task3() -> String {
        sleep(Duration::from_millis(150)).await;
        "Task 3 complete".to_string()
    }

    let start = std::time::Instant::now();

    let (r1, r2, r3) = tokio::join!(task1(), task2(), task3());

    println!("   Results:");
    println!("     {}", r1);
    println!("     {}", r2);
    println!("     {}", r3);
    println!("   Time: {:?}", start.elapsed());
    println!("   完成\n");

    Ok(())
}

/// select!: 竞争执行
async fn select_demo() -> anyhow::Result<()> {
    println!("2. select! (竞争执行)");

    use tokio::sync::mpsc;

    let (tx1, mut rx1) = mpsc::channel(100);
    let (tx2, mut rx2) = mpsc::channel(100);

    // 生产者 1
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx1.send("From channel 1").await.unwrap();
    });

    // 生产者 2
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx2.send("From channel 2").await.unwrap();
    });

    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("   Received from channel 1: {}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("   Received from channel 2: {}", msg);
        }
        else => {
            println!("   Both channels closed");
        }
    }

    println!("   完成\n");
    Ok(())
}

/// timeout: 超时控制
async fn timeout_demo() -> anyhow::Result<()> {
    println!("3. timeout (超时控制)");

    async fn slow_task() -> String {
        sleep(Duration::from_secs(2)).await;
        "Task completed".to_string()
    }

    // 尝试带超时的任务
    match timeout(Duration::from_secs(1), slow_task()).await {
        Ok(result) => println!("   Success: {}", result),
        Err(_) => println!("   Timeout after 1 second"),
    }

    // 任务在超时内完成
    async fn fast_task() -> String {
        sleep(Duration::from_millis(100)).await;
        "Task completed".to_string()
    }

    match timeout(Duration::from_secs(1), fast_task()).await {
        Ok(result) => println!("   Success: {}", result),
        Err(_) => println!("   Timeout"),
    }

    println!("   完成\n");
    Ok(())
}
```

- [ ] **Step 3: 更新 Cargo.toml**

```toml
futures = "0.3"
```

- [ ] **Step 4: 更新 main.rs**

修改 main.rs，将 `.await?` 改为 `.await` 并处理结果。

- [ ] **Step 5: 验证编译**

```bash
cargo check -p concurrent-tasks
```

- [ ] **Step 6: 提交**

```bash
git add demos/concurrent-tasks/src/
git commit -m "feat: implement threadpool and async concurrency examples"
```

---

## 第三阶段：集成服务

### Task 16: 创建 integrated-service 基础结构

**Files:**
- Create: `demos/integrated-service/Cargo.toml`
- Create: `demos/integrated-service/src/main.rs`
- Create: `demos/integrated-service/docker-compose.yml`
- Create: `demos/integrated-service/config.yml`
- Create: `demos/integrated-service/README.md`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "integrated-service"
version.workspace = true
edition.workspace = true

[dependencies]
# 共享模块
shared = { path = "../shared" }

# 工作区依赖
tokio.workspace = true
axum.workspace = true
tower.workspace = true
tower-http.workspace = true
sqlx.workspace = true
redis.workspace = true
rdkafka.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
anyhow.workspace = true
```

- [ ] **Step 2: 创建 Docker Compose**

```yaml
version: '3.8'

services:
  mysql:
    image: mysql:8.0
    container_name: rust_study_mysql
    environment:
      MYSQL_ROOT_PASSWORD: password
      MYSQL_DATABASE: rust_study
    ports:
      - "3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
    networks:
      - rust_study_net
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7
    container_name: rust_study_redis
    ports:
      - "6379:6379"
    networks:
      - rust_study_net
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    container_name: rust_study_zookeeper
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    networks:
      - rust_study_net

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    container_name: rust_study_kafka
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:29092,PLAINTEXT_HOST://localhost:9092
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: PLAINTEXT:PLAINTEXT,PLAINTEXT_HOST:PLAINTEXT
      KAFKA_INTER_BROKER_LISTENER_NAME: PLAINTEXT
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
      KAFKA_AUTO_CREATE_TOPICS_ENABLE: "true"
    networks:
      - rust_study_net

volumes:
  mysql_data:

networks:
  rust_study_net:
    driver: bridge
```

- [ ] **Step 3: 创建 Dockerfile（可选，用于容器化部署）**

**注意**: 此 Dockerfile 可选，主要用于容器化部署。本地开发可以直接运行 `cargo run`。

```dockerfile
# 多阶段构建，用于集成服务
FROM rust:1.83 as builder

WORKDIR /app

# 复制 Cargo 配置
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src
COPY demos ./demos

# 构建 integrated-service
RUN cargo build --release -p integrated-service

# 运行时镜像
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/integrated-service /app/integrated-service

EXPOSE 3000

# 设置环境变量
ENV APP_SERVER__HOST="0.0.0.0"
ENV APP_SERVER__PORT="3000"

CMD ["/app/integrated-service"]
```

- [ ] **Step 4: 创建基础 main.rs**

```rust
use axum::{Router, routing::get};
use shared::AppConfig;

#[tokio::main]
async fn main() -> shared::AppResult<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    let config = AppConfig::load(None)?;

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check));

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
```

- [ ] **Step 5: 创建配置文件**

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

- [ ] **Step 6: 创建 README**

```markdown
# Integrated Service

集成所有技术的完整服务示例。

## 功能

- HTTP API (Axum)
- MySQL 数据库 (SQLx)
- Redis 缓存
- Kafka 消息队列
- 完整的 CRUD 操作
- 事件驱动架构

## 启动服务

**使用 Docker Compose 启动依赖服务 (推荐):**

```bash
cd demos/integrated-service
docker-compose up -d mysql redis zookeeper kafka

# 等待服务启动（约30秒）
sleep 30

# 创建 Kafka topic
docker exec -it rust_study_kafka kafka-topics --create --topic user-events --bootstrap-server localhost:9092 --partitions 3 --replication-factor 1
```

**运行应用:**

```bash
cd demos/integrated-service
cargo run
```

应用将在 http://localhost:3000 启动。

## API 端点

- `GET /health` - 健康检查
- `GET /readiness` - 就绪检查（检查 MySQL、Redis 连接）
- `GET /api/users` - 获取用户列表
- `POST /api/users` - 创建用户
- `GET /api/users/:id` - 获取用户详情
- `PUT /api/users/:id` - 更新用户
- `DELETE /api/users/:id` - 删除用户
```

- [ ] **Step 7: 提交**

```bash
git add demos/integrated-service/
git commit -m "feat: create integrated-service base with docker compose"
```

---

### Task 17: 实现 integrated-service 完整功能

**Files:**
- Create: `demos/integrated-service/src/models.rs`
- Create: `demos/integrated-service/src/state.rs`
- Create: `demos/integrated-service/src/routes/mod.rs`
- Create: `demos/integrated-service/src/routes/users.rs`
- Create: `demos/integrated-service/src/middleware/mod.rs`
- Create: `demos/integrated-service/src/middleware/logging.rs`
- Modify: `demos/integrated-service/src/main.rs`

- [ ] **Step 1: 创建模型**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}
```

- [ ] **Step 2: 创建应用状态**

```rust
use sqlx::{MySql, Pool};
use redis::ConnectionManager;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Pool<MySql>,
    pub redis: Arc<ConnectionManager>,
    pub kafka_producer: Arc<rdkafka::producer::FutureProducer>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            redis: Arc::clone(&self.redis),
            kafka_producer: Arc::clone(&self.kafka_producer),
        }
    }
}
```

- [ ] **Step 3: 创建用户路由**

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::state::AppState;

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, shared::AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, shared::AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| shared::AppError::NotFound(format!("User {}", id)))?;

    Ok(Json(user))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<User>, shared::AppError> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO users (id, username, email, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(id)
    .bind(&req.username)
    .bind(&req.email)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    let user = User { id, username: req.username, email: req.email, created_at: now, updated_at: now };

    // 发送 Kafka 事件
    send_user_event(&state, "user.created", &user).await?;

    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>, shared::AppError> {
    let now = chrono::Utc::now();

    let mut query = String::from("UPDATE users SET updated_at = ?");
    let mut params = vec![];

    if req.username.is_some() {
        query.push_str(", username = ?");
        params.push(req.username.unwrap());
    }
    if req.email.is_some() {
        query.push_str(", email = ?");
        params.push(req.email.unwrap());
    }

    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query).bind(now);
    for param in params {
        q = q.bind(param);
    }
    q = q.bind(id);

    q.execute(&state.db).await?;

    get_user(State(state), Path(id)).await
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, shared::AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(shared::AppError::NotFound(format!("User {}", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn send_user_event(
    state: &AppState,
    event_type: &str,
    user: &User,
) -> Result<(), shared::AppError> {
    use serde_json::json;

    let event = json!({
        "event_type": event_type,
        "user_id": user.id,
        "username": &user.username,
        "email": &user.email,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    let payload = serde_json::to_vec(&event)?;
    let key = user.id.to_string();

    let record = rdkafka::producer::FutureRecord::to("user-events")
        .key(&key)
        .payload(&payload);

    state.kafka_producer
        .send(record, std::time::Duration::from_secs(5))
        .await
        .map_err(|e| shared::AppError::Messaging(format!("Failed to send event: {}", e.0)))?;

    Ok(())
}
```

- [ ] **Step 4: 创建日志中间件**

```rust
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info_span;

pub async fn logging_middleware(
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let span = info_span!(
        "http_request",
        method = %method,
        uri = %uri,
    );

    let response = next.run(req).instrument(span).await;

    response
}
```

- [ ] **Step 5: 更新 main.rs**

```rust
use axum::{Router, routing::get};
use shared::AppConfig;

mod models;
mod state;
mod routes;
mod middleware;

use middleware::logging_middleware;

#[tokio::main]
async fn main() -> shared::AppResult<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    let config = AppConfig::load(None)?;

    // 初始化依赖
    let db = init_db(&config).await?;
    let redis = init_redis(&config).await?;
    let kafka = init_kafka(&config)?;

    let state = state::AppState {
        db,
        redis: std::sync::Arc::new(redis),
        kafka_producer: std::sync::Arc::new(kafka),
    };

    // 运行迁移
    run_migrations(&state.db).await?;

    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/readiness", get(readiness_check))
        .nest("/api/users", routes::users::routes())
        .layer(axum::middleware::from_fn(logging_middleware))
        .with_state(state.clone());

    // 设置优雅关闭
    let state_clone = state.clone();
    let shutdown_signal = async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C handler");
        tracing::info!("Received shutdown signal");
        // 清理资源
        drop(state_clone);
    };

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn init_db(config: &AppConfig) -> shared::AppResult<sqlx::Pool<sqlx::MySql>> {
    let db_config = config.database.as_ref().expect("Database config required");
    let pool = sqlx::MySqlPool::connect_with(
        sqlx::mysql::MySqlConnectOptions::from_str(&db_config.url)?
            .max_connections(db_config.max_connections)
    ).await?;

    tracing::info!("Connected to MySQL");
    Ok(pool)
}

async fn init_redis(config: &AppConfig) -> shared::AppResult<redis::ConnectionManager> {
    let redis_config = config.redis.as_ref().expect("Redis config required");
    let client = redis::Client::open(&redis_config.url)?;
    let manager = redis::ConnectionManager::new(client).await?;

    tracing::info!("Connected to Redis");
    Ok(manager)
}

fn init_kafka(config: &AppConfig) -> shared::AppResult<rdkafka::producer::FutureProducer> {
    let kafka_config = config.kafka.as_ref().expect("Kafka config required");

    let producer: rdkafka::producer::FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", &kafka_config.bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()?;

    tracing::info!("Connected to Kafka");
    Ok(producer)
}

async fn run_migrations(pool: &sqlx::Pool<sqlx::MySql>) -> shared::AppResult<()> {
    let create_users = tokio::fs::read_to_string("../mysql-crud/migrations/001_create_users.sql").await?;
    sqlx::query(&create_users).execute(pool).await?;

    tracing::info!("Migrations completed");
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn readiness_check(State(state): State<AppState>) -> Result<Json<serde_json::Value>, shared::AppError> {
    // 检查 MySQL 连接
    sqlx::query("SELECT 1").fetch_one(&state.db).await
        .map_err(|e| shared::AppError::Database(format!("MySQL not ready: {}", e)))?;

    // 检查 Redis 连接
    {
        let mut conn = state.redis.clone();
        redis::cmd("PING").query_async(&mut conn).await
            .map_err(|e| shared::AppError::Cache(format!("Redis not ready: {}", e)))?;
    }

    Ok(Json(serde_json::json!({
        "status": "ready",
        "mysql": "ok",
        "redis": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
```

- [ ] **Step 6: 验证编译**

```bash
cargo check -p integrated-service
```

- [ ] **Step 7: 提交**

```bash
git add demos/integrated-service/src/
git commit -m "feat: implement full integrated-service with all components"
```

---

## 第四阶段：完善和文档

### Task 18: 编写主 README

**Files:**
- Modify: `README.md` (root)

- [ ] **Step 1: 更新主 README**

```markdown
# Rust Study 项目

这是一个全面的 Rust 学习项目，从基础语法到生产级应用示例。

## 项目结构

```
rust_study/
├── src/                  # 基础语法学习
│   ├── basics/           # 变量、数据类型、函数
│   ├── control_flow/     # 流程控制
│   ├── ownership/        # 所有权和借用
│   ├── structs_enums/    # 结构体、枚举
│   ├── collections/      # 集合类型
│   ├── modules_errors/   # 模块和错误处理
│   └── advanced/         # 高级特性
│
└── demos/                # 生产级 demo
    ├── http-api/         # RESTful API 服务
    ├── mysql-crud/       # MySQL 数据库操作
    ├── redis-cache/      # Redis 缓存服务
    ├── kafka-messaging/  # Kafka 消息队列
    ├── concurrent-tasks/ # 并发编程
    └── integrated-service/ # 集成服务
```

## 学习路径

### 第一阶段：基础语法

```bash
cd rust_study
cargo run
```

按菜单选择不同的主题学习：
1. 基础语法（变量、数据类型、函数）
2. 流程控制（if-else、循环）
3. 所有权和借用
4. 结构体、枚举和模式匹配
5. 集合（向量、字符串、哈希映射）
6. 模块和错误处理
7. 高级特性（泛型、Trait、生命周期）

### 第二阶段：生产级应用

每个 demo 都是独立的 crate，可以单独学习和运行。

#### 1. HTTP API 服务

```bash
cd demos/http-api
cargo run
```

学习：
- Axum 框架
- RESTful API 设计
- 中间件
- 错误处理

#### 2. MySQL 数据库

```bash
cd demos/mysql-crud
# 先启动 MySQL
cargo run
```

学习：
- SQLx
- 数据库连接池
- CRUD 操作
- 事务处理

#### 3. Redis 缓存

```bash
cd demos/redis-cache
# 先启动 Redis
cargo run
```

学习：
- Redis 操作
- 缓存策略
- 缓存防护

#### 4. Kafka 消息队列

```bash
cd demos/kafka-messaging
# 先启动 Kafka

# 启动 producer
cargo run --bin producer

# 启动 consumer
cargo run --bin consumer
```

学习：
- Kafka 消息发送和消费
- 消费者组
- 序列化

#### 5. 并发编程

```bash
cd demos/concurrent-tasks
cargo run
```

学习：
- Channel 通信
- 共享状态
- 线程池
- 异步并发

#### 6. 集成服务

```bash
cd demos/integrated-service
# 使用 Docker Compose 启动所有依赖
docker-compose up -d

# 运行服务
cargo run
```

综合应用所有技术。

## 技术栈

- **HTTP**: Axum 0.8
- **异步运行时**: Tokio 1.40
- **数据库**: SQLx 0.8
- **缓存**: Redis 0.26
- **消息队列**: rust-rdkafka 0.36
- **序列化**: serde
- **日志**: tracing

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可

MIT
```

- [ ] **Step 2: 提交**

```bash
git add README.md
git commit -m "docs: add comprehensive README for the project"
```

---

### Task 19: 运行完整测试

- [ ] **Step 1: 运行所有测试**

```bash
cargo test --workspace
```

Expected: 所有测试通过

- [ ] **Step 2: 检查所有 demo 编译**

```bash
cargo check --workspace
```

Expected: 所有 crate 编译成功

- [ ] **Step 3: 格式化代码**

```bash
cargo fmt --all
```

- [ ] **Step 4: 运行 clippy**

```bash
cargo clippy --workspace -- -D warnings
```

修复所有警告

- [ ] **Step 5: 提交**

```bash
git add .
git commit -m "chore: run tests and fix warnings"
```

---

### Task 20: 最终验证和文档完善

- [ ] **Step 1: 验证每个 demo 可运行**

```bash
# HTTP API
cd demos/http-api && cargo run &

# MySQL CRUD (需要 MySQL)
cd demos/mysql-crud && cargo run

# Redis Cache (需要 Redis)
cd demos/redis-cache && cargo run

# Kafka (需要 Kafka)
cd demos/kafka-messaging && cargo run --bin producer & cargo run --bin consumer

# Concurrent Tasks
cd demos/concurrent-tasks && cargo run

# Integrated Service (需要所有依赖)
cd demos/integrated-service && docker-compose up
```

- [ ] **Step 2: 检查所有 README 文件**

确保每个 demo 的 README 完整清晰

- [ ] **Step 3: 创建快速启动脚本**

创建 `scripts/start-all.sh`:

```bash
#!/bin/bash

# 启动所有依赖服务
docker-compose -f demos/integrated-service/docker-compose.yml up -d mysql redis zookeeper kafka

# 等待服务启动
sleep 10

echo "所有服务已启动"
echo "MySQL: localhost:3306"
echo "Redis: localhost:6379"
echo "Kafka: localhost:9092"
```

- [ ] **Step 4: 创建开发文档**

创建 `docs/development.md`:

```markdown
# 开发指南

## 环境设置

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装 Docker

用于运行依赖服务。

### 配置

复制并修改配置文件：

```bash
cp demos/shared/config.example.yml config.yml
```

## 开发工作流

### 添加新功能

1. 创建分支
2. 编写测试
3. 实现功能
4. 运行测试
5. 提交

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 所有公共函数必须有文档注释
- 编写测试覆盖核心逻辑

## 调试

### 查看日志

日志会输出到 stdout，使用 tracing 支持。

### 数据库连接

使用 MySQL 客户端连接：

```bash
mysql -h localhost -P 3306 -u root -p
```

### Redis 连接

```bash
redis-cli
```
```

- [ ] **Step 5: 最终提交**

```bash
git add .
git commit -m "docs: finalize documentation and add development guide"
```

---

## 总结

本实施计划涵盖了：

1. ✅ Cargo Workspace 配置
2. ✅ 共享模块（错误处理、配置）
3. ✅ 6 个生产级 demo
   - http-api
   - mysql-crud
   - redis-cache
   - kafka-messaging
   - concurrent-tasks
   - integrated-service
4. ✅ Docker Compose 配置
5. ✅ 完整文档

每个任务都是独立的，可以逐步完成。建议按顺序实施，确保每个步骤都正常运行后再继续下一步。
