//! 配置管理模块
//!
//! 支持从环境变量读取配置，提供合理的默认值

use serde::Deserialize;

/// 应用配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// 服务监听地址
    #[serde(default = "default_host")]
    pub host: String,

    /// 服务监听端口
    #[serde(default = "default_port")]
    pub port: u16,

    /// 数据库连接地址
    #[serde(default = "default_database_url")]
    pub database_url: String,

    /// Redis 连接地址
    #[serde(default = "default_redis_url")]
    pub redis_url: String,

    /// Kafka broker 地址
    #[serde(default = "default_kafka_brokers")]
    pub kafka_brokers: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_database_url() -> String {
    "mysql://root:password@localhost:3306/rust_study".to_string()
}

fn default_redis_url() -> String {
    "redis://localhost:6379".to_string()
}

fn default_kafka_brokers() -> String {
    "localhost:9092".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            database_url: default_database_url(),
            redis_url: default_redis_url(),
            kafka_brokers: default_kafka_brokers(),
        }
    }
}

impl AppConfig {
    /// 从环境变量加载配置
    ///
    /// 环境变量对应关系：
    /// - APP_HOST -> host
    /// - APP_PORT -> port
    /// - DATABASE_URL -> database_url
    /// - REDIS_URL -> redis_url
    /// - KAFKA_BROKERS -> kafka_brokers
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("APP_HOST").unwrap_or_else(|_| default_host()),
            port: std::env::var("APP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or_else(default_port),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| default_database_url()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| default_redis_url()),
            kafka_brokers: std::env::var("KAFKA_BROKERS")
                .unwrap_or_else(|_| default_kafka_brokers()),
        }
    }

    /// 获取服务监听地址
    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert!(config.database_url.contains("mysql://"));
        assert!(config.redis_url.contains("redis://"));
        assert!(config.kafka_brokers.contains("9092"));
    }

    #[test]
    fn test_listen_addr() {
        let config = AppConfig::default();
        assert_eq!(config.listen_addr(), "0.0.0.0:3000");

        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            ..AppConfig::default()
        };
        assert_eq!(config.listen_addr(), "127.0.0.1:8080");
    }

    #[test]
    fn test_from_env() {
        // from_env 使用默认值（未设置环境变量时）
        let config = AppConfig::from_env();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_from_env_with_vars() {
        // 设置环境变量 (edition 2024 中 set_var 是 unsafe)
        unsafe {
            std::env::set_var("APP_HOST", "192.168.1.1");
            std::env::set_var("APP_PORT", "9090");
        }
        let config = AppConfig::from_env();
        assert_eq!(config.host, "192.168.1.1");
        assert_eq!(config.port, 9090);
        // 清理
        unsafe {
            std::env::remove_var("APP_HOST");
            std::env::remove_var("APP_PORT");
        }
    }

    #[test]
    fn test_from_env_invalid_port() {
        unsafe { std::env::set_var("APP_PORT", "not_a_number"); }
        let config = AppConfig::from_env();
        assert_eq!(config.port, 3000); // 应该回退到默认值
        unsafe { std::env::remove_var("APP_PORT"); }
    }

    #[test]
    fn test_config_clone() {
        let config = AppConfig::default();
        let cloned = config.clone();
        assert_eq!(config.host, cloned.host);
        assert_eq!(config.port, cloned.port);
    }

    #[test]
    fn test_config_debug() {
        let config = AppConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("AppConfig"));
        assert!(debug.contains("3000"));
    }
}
