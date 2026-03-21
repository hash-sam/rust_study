//! Integrated Service Demo - 整合 HTTP + MySQL + Redis + Kafka
//!
//! 一个完整的用户管理微服务，展示多组件协作:
//! - Axum HTTP API
//! - MySQL 持久化存储
//! - Redis 缓存加速
//! - Kafka 事件通知
//! - 健康检查、优雅关闭

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};

// ========== 数据模型 ==========

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ApiResp<T: Serialize> {
    pub code: u16,
    pub data: Option<T>,
    pub msg: String,
}

impl<T: Serialize> ApiResp<T> {
    fn ok(data: T) -> Json<Self> {
        Json(Self {
            code: 200,
            data: Some(data),
            msg: "ok".into(),
        })
    }
}
impl ApiResp<()> {
    fn err(code: u16, msg: impl Into<String>) -> Json<Self> {
        Json(Self {
            code,
            data: None,
            msg: msg.into(),
        })
    }
}

// ========== 应用状态 ==========

pub struct AppState {
    db: MySqlPool,
    redis: redis::aio::MultiplexedConnection,
    kafka: Option<rdkafka::producer::FutureProducer>,
}

type SharedState = Arc<AppState>;

// ========== 错误处理 ==========

enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, ApiResp::<()>::err(status.as_u16(), msg)).into_response()
    }
}

// ========== Handlers ==========

async fn health(State(state): State<SharedState>) -> impl IntoResponse {
    let db_ok = sqlx::query("SELECT 1").execute(&state.db).await.is_ok();
    let redis_ok: bool = redis::cmd("PING")
        .query_async(&mut state.redis.clone())
        .await
        .map(|r: String| r == "PONG")
        .unwrap_or(false);
    Json(serde_json::json!({
        "status": if db_ok && redis_ok { "healthy" } else { "degraded" },
        "mysql": db_ok,
        "redis": redis_ok,
        "kafka": state.kafka.is_some(),
    }))
}

async fn create_user(
    State(state): State<SharedState>,
    Json(req): Json<CreateUserReq>,
) -> Result<impl IntoResponse, AppError> {
    if req.name.is_empty() {
        return Err(AppError::BadRequest("name 不能为空".into()));
    }
    if !req.email.contains('@') {
        return Err(AppError::BadRequest("email 格式错误".into()));
    }

    // 写入 MySQL
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&req.name)
        .bind(&req.email)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(format!("DB写入失败: {}", e)))?;
    let id = result.last_insert_id() as i64;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // 写入 Redis 缓存
    let cache_key = format!("user:{}", id);
    let json = serde_json::to_string(&user).unwrap();
    let _: redis::RedisResult<()> = state.redis.clone().set_ex(&cache_key, &json, 300).await;
    info!("创建用户: id={}, 已缓存", id);

    // 发送 Kafka 事件
    if let Some(ref producer) = state.kafka {
        let event = serde_json::json!({"type": "user.created", "user_id": id, "name": &req.name});
        let payload = event.to_string();
        let key = id.to_string();
        let record = rdkafka::producer::FutureRecord::to("user-events")
            .key(&key)
            .payload(&payload);
        if let Err((e, _)) = producer
            .send(record, std::time::Duration::from_secs(3))
            .await
        {
            warn!("Kafka事件发送失败: {}", e);
        }
    }

    Ok((StatusCode::CREATED, ApiResp::ok(user)))
}

async fn get_user(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let cache_key = format!("user:{}", id);

    // 1. 先查 Redis
    let cached: redis::RedisResult<Option<String>> = state.redis.clone().get(&cache_key).await;
    if let Ok(Some(json)) = cached {
        if let Ok(user) = serde_json::from_str::<User>(&json) {
            info!("缓存命中: user:{}", id);
            return Ok(ApiResp::ok(user));
        }
    }

    // 2. 缓存未命中，查 MySQL
    info!("缓存未命中, 查DB: user:{}", id);
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("用户 {} 不存在", id)))?;

    // 3. 回填缓存
    let json = serde_json::to_string(&user).unwrap();
    let _: redis::RedisResult<()> = state.redis.clone().set_ex(&cache_key, &json, 300).await;

    Ok(ApiResp::ok(user))
}

async fn list_users(State(state): State<SharedState>) -> Result<impl IntoResponse, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id DESC LIMIT 50")
        .fetch_all(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(ApiResp::ok(users))
}

async fn delete_user(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("用户 {} 不存在", id)));
    }

    // 清除缓存
    let _: redis::RedisResult<()> = state.redis.clone().del(format!("user:{}", id)).await;

    // Kafka 事件
    if let Some(ref producer) = state.kafka {
        let event = serde_json::json!({"type": "user.deleted", "user_id": id});
        let payload = event.to_string();
        let key = id.to_string();
        let record = rdkafka::producer::FutureRecord::to("user-events")
            .key(&key)
            .payload(&payload);
        let _ = producer
            .send(record, std::time::Duration::from_secs(3))
            .await;
    }

    info!("删除用户: id={}, 已清缓存", id);
    Ok(ApiResp::ok(format!("用户 {} 已删除", id)))
}

// ========== 路由 ==========

fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/{id}", get(get_user).delete(delete_user))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

// ========== main ==========

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("integrated_service=debug,tower_http=debug,sqlx=warn")
        .init();
    let config = shared::AppConfig::from_env();
    info!("Integrated Service 启动");

    // MySQL
    let db = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(p) => {
            info!("MySQL 连接成功");
            p
        }
        Err(e) => {
            error!("MySQL 连接失败: {}. 请 docker-compose up -d", e);
            return;
        }
    };
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGINT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(100) NOT NULL,
            email VARCHAR(200) NOT NULL UNIQUE,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
    )
    .execute(&db)
    .await
    .unwrap();

    // Redis
    let redis = match redis::Client::open(config.redis_url.as_str()) {
        Ok(c) => match c.get_multiplexed_async_connection().await {
            Ok(conn) => {
                info!("Redis 连接成功");
                conn
            }
            Err(e) => {
                error!("Redis 连接失败: {}", e);
                return;
            }
        },
        Err(e) => {
            error!("Redis URL 错误: {}", e);
            return;
        }
    };

    // Kafka (可选)
    let kafka = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", &config.kafka_brokers)
        .set("message.timeout.ms", "3000")
        .create::<rdkafka::producer::FutureProducer>()
        .ok();
    if kafka.is_some() {
        info!("Kafka 连接成功");
    } else {
        warn!("Kafka 未连接, 事件功能禁用");
    }

    let state = Arc::new(AppState { db, redis, kafka });
    let app = build_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    info!("服务监听: http://{}", addr);
    info!("API: GET/POST /api/users, GET/DELETE /api/users/:id, GET /health");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            info!("优雅关闭中...");
        })
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use tower::ServiceExt;

    /// 尝试构建完整的 AppState（需要 MySQL + Redis）
    async fn build_test_state() -> Option<SharedState> {
        let config = shared::AppConfig::from_env();

        let db = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(2)
            .connect(&config.database_url)
            .await
            .ok()?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id BIGINT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(100) NOT NULL,
                email VARCHAR(200) NOT NULL UNIQUE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4",
        )
        .execute(&db)
        .await
        .ok()?;

        let redis = redis::Client::open(config.redis_url.as_str())
            .ok()?
            .get_multiplexed_async_connection()
            .await
            .ok()?;

        let kafka = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", &config.kafka_brokers)
            .set("message.timeout.ms", "2000")
            .create::<rdkafka::producer::FutureProducer>()
            .ok();

        Some(Arc::new(AppState { db, redis, kafka }))
    }

    macro_rules! skip_if_no_services {
        ($state:expr) => {
            match $state {
                Some(s) => s,
                None => {
                    eprintln!("跳过: MySQL/Redis 不可用");
                    return;
                }
            }
        };
    }

    // ========== 单元测试 (无需外部服务) ==========

    #[test]
    fn test_api_resp_ok() {
        let resp = ApiResp::ok(42);
        let json = resp.0; // 取出 Json 内部值
        assert_eq!(json.code, 200);
        assert_eq!(json.data, Some(42));
    }

    #[test]
    fn test_api_resp_err() {
        let resp = ApiResp::<()>::err(404, "not found");
        let json = resp.0;
        assert_eq!(json.code, 404);
        assert!(json.data.is_none());
    }

    #[test]
    fn test_create_user_req_deserialize() {
        let json = r#"{"name":"test","email":"t@t.com"}"#;
        let req: CreateUserReq = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "test");
        assert_eq!(req.email, "t@t.com");
    }

    #[test]
    fn test_user_serialize() {
        let user = User {
            id: 1,
            name: "test".into(),
            email: "t@t.com".into(),
            created_at: chrono::NaiveDateTime::parse_from_str(
                "2024-01-01 00:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("t@t.com"));
    }

    // ========== 集成测试 (需要 MySQL + Redis) ==========

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = skip_if_no_services!(build_test_state().await);
        let app = build_router(state);
        let req = axum::http::Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let state = skip_if_no_services!(build_test_state().await);
        let app = build_router(state);

        let email = format!("integ_test_{}@test.com", uuid::Uuid::new_v4());
        let body = serde_json::json!({"name": "集成测试", "email": email});

        // Create
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::CREATED);

        let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let id = json["data"]["id"].as_i64().unwrap();

        // Get (第一次走 Redis 缓存)
        let req = axum::http::Request::builder()
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        // Delete
        let req = axum::http::Request::builder()
            .method("DELETE")
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        // Get after delete
        let req = axum::http::Request::builder()
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_bad_request() {
        let state = skip_if_no_services!(build_test_state().await);
        let app = build_router(state);

        // 空 name
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"","email":"a@b.com"}"#))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);

        // 无效 email
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"test","email":"invalid"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_list_users() {
        let state = skip_if_no_services!(build_test_state().await);
        let app = build_router(state);

        let req = axum::http::Request::builder()
            .uri("/api/users")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
    }
}
