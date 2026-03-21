//! 共享模块 - 错误处理、配置管理、日志等

pub mod error;
pub mod config;

pub use error::{AppError, AppResult};
pub use config::AppConfig;
