//! 统一错误处理模块
//!
//! 提供全局统一的错误类型 AppError，支持：
//! - 自动转换常见错误类型（sqlx, redis, serde_json, io）
//! - 与 axum 集成，自动转为 HTTP 响应
//! - 结构化日志记录

use thiserror::Error;

/// 应用统一错误类型
#[derive(Debug, Error)]
pub enum AppError {
    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),

    /// 请求参数无效
    #[error("无效请求: {0}")]
    BadRequest(String),

    /// 认证失败
    #[error("认证失败: {0}")]
    Unauthorized(String),

    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    /// Redis 错误
    #[error("Redis错误: {0}")]
    Redis(#[from] redis::RedisError),

    /// JSON 序列化/反序列化错误
    #[error("JSON错误: {0}")]
    Json(#[from] serde_json::Error),

    /// IO 错误
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    /// 内部错误（兜底）
    #[error("内部错误: {0}")]
    Internal(String),
}

/// 应用统一 Result 类型
pub type AppResult<T> = Result<T, AppError>;

/// 便捷方法
impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        AppError::Internal(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        AppError::NotFound(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        AppError::BadRequest(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = AppError::NotFound("用户123".to_string());
        assert!(e.to_string().contains("用户123"));

        let e = AppError::BadRequest("参数错误".to_string());
        assert!(e.to_string().contains("参数错误"));

        let e = AppError::Internal("服务异常".to_string());
        assert!(e.to_string().contains("服务异常"));
    }

    #[test]
    fn test_error_constructors() {
        let e = AppError::not_found("not found");
        assert!(matches!(e, AppError::NotFound(_)));

        let e = AppError::bad_request("bad request");
        assert!(matches!(e, AppError::BadRequest(_)));

        let e = AppError::internal("internal");
        assert!(matches!(e, AppError::Internal(_)));
    }

    #[test]
    fn test_error_from_json() {
        let bad_json = serde_json::from_str::<serde_json::Value>("invalid");
        assert!(bad_json.is_err());
        let err: AppError = bad_json.unwrap_err().into();
        assert!(matches!(err, AppError::Json(_)));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在");
        let err: AppError = io_err.into();
        assert!(matches!(err, AppError::Io(_)));
        assert!(err.to_string().contains("文件不存在"));
    }

    #[test]
    fn test_error_debug() {
        let e = AppError::internal("test");
        let debug = format!("{:?}", e);
        assert!(debug.contains("Internal"));
    }

    #[test]
    fn test_app_result_ok() {
        let result: AppResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_app_result_err() {
        let result: AppResult<i32> = Err(AppError::not_found("missing"));
        assert!(result.is_err());
    }
}
