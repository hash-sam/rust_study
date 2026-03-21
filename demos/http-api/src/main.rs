//! HTTP API Demo - 基于 Axum 的 RESTful API 服务
//!
//! 演示: 路由、中间件、JSON处理、状态共享、错误处理、优雅关闭

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

// ========== 数据模型 ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}
fn default_page() -> u64 { 1 }
fn default_per_page() -> u64 { 10 }

/// 统一 API 响应
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { code: 200, message: "success".into(), data: Some(data) }
    }
}

impl ApiResponse<()> {
    pub fn error(code: u16, msg: impl Into<String>) -> Self {
        Self { code, message: msg.into(), data: None }
    }
}

// ========== 应用状态 ==========

pub struct AppState {
    users: RwLock<HashMap<u64, User>>,
    next_id: RwLock<u64>,
}

// ========== 错误处理 ==========

pub enum ApiError {
    NotFound(String),
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
        };
        (status, Json(ApiResponse::<()>::error(status.as_u16(), msg))).into_response()
    }
}

// ========== Handler ==========

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "http-api-demo" }))
}

async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let users = state.users.read().await;
    let mut list: Vec<&User> = users.values().collect();
    list.sort_by_key(|u| u.id);
    let total = list.len() as u64;
    let start = ((params.page - 1) * params.per_page) as usize;
    let page: Vec<User> = list.into_iter().skip(start).take(params.per_page as usize).cloned().collect();
    info!("查询用户列表: page={}, total={}", params.page, total);
    Json(serde_json::json!({ "code": 200, "data": page, "total": total }))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, ApiError> {
    let users = state.users.read().await;
    users.get(&id)
        .cloned()
        .map(|u| Json(ApiResponse::success(u)))
        .ok_or_else(|| ApiError::NotFound(format!("用户 {} 不存在", id)))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if req.name.is_empty() {
        return Err(ApiError::BadRequest("用户名不能为空".into()));
    }
    if !req.email.contains('@') {
        return Err(ApiError::BadRequest("邮箱格式不正确".into()));
    }
    let mut next_id = state.next_id.write().await;
    let id = *next_id;
    *next_id += 1;
    let user = User { id, name: req.name, email: req.email, age: req.age };
    state.users.write().await.insert(id, user.clone());
    info!("创建用户: id={}, name={}", user.id, user.name);
    Ok((StatusCode::CREATED, Json(ApiResponse::success(user))))
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut users = state.users.write().await;
    let user = users.get_mut(&id).ok_or_else(|| ApiError::NotFound(format!("用户 {} 不存在", id)))?;
    if let Some(name) = req.name { user.name = name; }
    if let Some(email) = req.email { user.email = email; }
    if let Some(age) = req.age { user.age = Some(age); }
    info!("更新用户: id={}", id);
    Ok(Json(ApiResponse::success(user.clone())))
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, ApiError> {
    let removed = state.users.write().await.remove(&id);
    match removed {
        Some(u) => {
            warn!("删除用户: id={}, name={}", u.id, u.name);
            Ok(Json(ApiResponse::success(format!("用户 {} 已删除", u.name))))
        }
        None => Err(ApiError::NotFound(format!("用户 {} 不存在", id))),
    }
}

// ========== 路由 ==========

fn build_router(state: Arc<AppState>) -> Router {
    let user_routes = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user));

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/users", user_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("http_api=debug,tower_http=debug")
        .init();

    let state = Arc::new(AppState {
        users: RwLock::new(HashMap::new()),
        next_id: RwLock::new(1),
    });

    let app = build_router(state);
    let addr = "0.0.0.0:3000";
    info!("HTTP API 服务启动: http://{}", addr);
    info!("健康检查: GET /health");
    info!("用户API: GET/POST /api/users, GET/PUT/DELETE /api/users/:id");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            info!("收到关闭信号，优雅关闭中...");
        })
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    /// 构建测试用的 app
    fn test_app() -> Router {
        let state = Arc::new(AppState {
            users: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        });
        build_router(state)
    }

    /// 从 Body 读取 JSON
    async fn body_json(body: Body) -> serde_json::Value {
        let bytes = body.collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"张三","email":"zs@test.com","age":28}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"]["name"], "张三");
        assert_eq!(json["data"]["email"], "zs@test.com");
        assert_eq!(json["data"]["id"], 1);
    }

    #[tokio::test]
    async fn test_create_user_bad_request_empty_name() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"","email":"a@b.com"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_user_bad_email() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"test","email":"invalid"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .uri("/api/users/999")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_crud_full_flow() {
        let state = Arc::new(AppState {
            users: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        });
        let app = build_router(state);

        // Create
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"李四","email":"ls@test.com"}"#))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let json = body_json(resp.into_body()).await;
        let id = json["data"]["id"].as_u64().unwrap();

        // Read
        let req = axum::http::Request::builder()
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"]["name"], "李四");

        // Update
        let req = axum::http::Request::builder()
            .method("PUT")
            .uri(format!("/api/users/{}", id))
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"李四(更新)"}"#))
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"]["name"], "李四(更新)");

        // List
        let req = axum::http::Request::builder()
            .uri("/api/users")
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"].as_array().unwrap().len(), 1);

        // Delete
        let req = axum::http::Request::builder()
            .method("DELETE")
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify deleted
        let req = axum::http::Request::builder()
            .uri(format!("/api/users/{}", id))
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_list_with_pagination() {
        let state = Arc::new(AppState {
            users: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
        });
        let app = build_router(state);

        // 创建 3 个用户
        for i in 1..=3 {
            let req = axum::http::Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("Content-Type", "application/json")
                .body(Body::from(format!(r#"{{"name":"user{}","email":"u{}@t.com"}}"#, i, i)))
                .unwrap();
            app.clone().oneshot(req).await.unwrap();
        }

        // 分页：每页2条
        let req = axum::http::Request::builder()
            .uri("/api/users?page=1&per_page=2")
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"].as_array().unwrap().len(), 2);
        assert_eq!(json["total"], 3);

        // 第 2 页
        let req = axum::http::Request::builder()
            .uri("/api/users?page=2&per_page=2")
            .body(Body::empty())
            .unwrap();
        let resp = app.clone().oneshot(req).await.unwrap();
        let json = body_json(resp.into_body()).await;
        assert_eq!(json["data"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .method("DELETE")
            .uri("/api/users/999")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let app = test_app();
        let req = axum::http::Request::builder()
            .method("PUT")
            .uri("/api/users/999")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"name":"x"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_api_response_format() {
        let resp = ApiResponse::success(42);
        assert_eq!(resp.code, 200);
        assert_eq!(resp.message, "success");
        assert_eq!(resp.data, Some(42));
    }

    #[tokio::test]
    async fn test_api_response_error_format() {
        let resp = ApiResponse::<()>::error(404, "not found");
        assert_eq!(resp.code, 404);
        assert_eq!(resp.message, "not found");
        assert!(resp.data.is_none());
    }
}
