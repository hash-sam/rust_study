//! MySQL CRUD Demo - 基于 SQLx 的数据库操作
//!
//! 演示: 连接池、CRUD、事务、JOIN查询、分页、聚合

use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::{FromRow, Row};
use tracing::{error, info, warn};

// ========== 数据模型 ==========

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserArticle {
    pub article_id: i64,
    pub title: String,
    pub author_name: String,
    pub author_email: String,
}

// ========== 数据库初始化 ==========

async fn create_pool(url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .idle_timeout(std::time::Duration::from_secs(300))
        .connect(url)
        .await?;
    info!("数据库连接池已创建");
    Ok(pool)
}

async fn init_tables(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(200) NOT NULL UNIQUE,
            age INT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            INDEX idx_email (email)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
    ).execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS articles (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(500) NOT NULL,
            content TEXT NOT NULL,
            author_id BIGINT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4"
    ).execute(pool).await?;

    info!("数据库表初始化完成");
    Ok(())
}

// ========== CRUD ==========

async fn insert_user(pool: &MySqlPool, name: &str, email: &str, age: Option<i32>) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO users (name, email, age) VALUES (?, ?, ?)")
        .bind(name).bind(email).bind(age)
        .execute(pool).await?;
    let id = result.last_insert_id() as i64;
    info!("插入用户: id={}, name={}", id, name);
    Ok(id)
}

async fn get_user_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id).fetch_optional(pool).await
}

async fn list_users(pool: &MySqlPool, page: u32, per_page: u32) -> Result<Vec<User>, sqlx::Error> {
    let offset = (page - 1) * per_page;
    sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id LIMIT ? OFFSET ?")
        .bind(per_page).bind(offset)
        .fetch_all(pool).await
}

async fn update_user(pool: &MySqlPool, id: i64, name: &str, email: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
        .bind(name).bind(email).bind(id)
        .execute(pool).await?;
    let ok = result.rows_affected() > 0;
    if ok { info!("更新用户: id={}", id); } else { warn!("用户不存在: id={}", id); }
    Ok(ok)
}

async fn delete_user(pool: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id).execute(pool).await?;
    Ok(result.rows_affected() > 0)
}

// ========== 高级操作 ==========

/// 事务：创建用户并批量插入文章
async fn create_user_with_articles(
    pool: &MySqlPool, name: &str, email: &str, articles: Vec<(&str, &str)>,
) -> Result<i64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    info!("开始事务: 创建用户及文章");

    let user_result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(name).bind(email).execute(&mut *tx).await?;
    let user_id = user_result.last_insert_id() as i64;

    for (title, content) in &articles {
        sqlx::query("INSERT INTO articles (title, content, author_id) VALUES (?, ?, ?)")
            .bind(title).bind(content).bind(user_id)
            .execute(&mut *tx).await?;
    }

    tx.commit().await?;
    info!("事务提交: user_id={}, articles={}", user_id, articles.len());
    Ok(user_id)
}

/// JOIN 查询
async fn get_user_articles(pool: &MySqlPool) -> Result<Vec<UserArticle>, sqlx::Error> {
    sqlx::query_as::<_, UserArticle>(
        "SELECT a.id as article_id, a.title, u.name as author_name, u.email as author_email
         FROM articles a INNER JOIN users u ON a.author_id = u.id ORDER BY a.created_at DESC"
    ).fetch_all(pool).await
}

/// 模糊搜索
async fn search_users(pool: &MySqlPool, keyword: &str) -> Result<Vec<User>, sqlx::Error> {
    let pattern = format!("%{}%", keyword);
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE name LIKE ? OR email LIKE ?")
        .bind(&pattern).bind(&pattern)
        .fetch_all(pool).await
}

/// 聚合：每个用户的文章数
async fn count_articles_per_user(pool: &MySqlPool) -> Result<Vec<(String, i64)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT u.name, COUNT(a.id) as cnt FROM users u LEFT JOIN articles a ON u.id = a.author_id GROUP BY u.id, u.name ORDER BY cnt DESC"
    ).fetch_all(pool).await?;
    Ok(rows.iter().map(|r| (r.get("name"), r.get("cnt"))).collect())
}

// ========== main ==========

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_env_filter("mysql_crud=debug,sqlx=warn").init();
    let config = shared::AppConfig::from_env();
    info!("MySQL CRUD Demo 启动, db={}", config.database_url);

    let pool = match create_pool(&config.database_url).await {
        Ok(p) => p,
        Err(e) => { error!("数据库连接失败: {}. 请 docker-compose up -d", e); return; }
    };
    if let Err(e) = init_tables(&pool).await { error!("建表失败: {}", e); return; }

    // 清理旧数据
    let _ = sqlx::query("DELETE FROM articles").execute(&pool).await;
    let _ = sqlx::query("DELETE FROM users").execute(&pool).await;

    println!("\n=== 基础 CRUD ===");
    let id1 = insert_user(&pool, "张三", "zhangsan@example.com", Some(28)).await.unwrap();
    let id2 = insert_user(&pool, "李四", "lisi@example.com", Some(32)).await.unwrap();
    let id3 = insert_user(&pool, "王五", "wangwu@example.com", None).await.unwrap();
    println!("创建 3 个用户: [{}, {}, {}]", id1, id2, id3);

    if let Some(u) = get_user_by_id(&pool, id1).await.unwrap() {
        println!("查询用户{}: {} <{}>", id1, u.name, u.email);
    }

    let page = list_users(&pool, 1, 2).await.unwrap();
    println!("分页(1,2): {:?}", page.iter().map(|u| &u.name).collect::<Vec<_>>());

    update_user(&pool, id1, "张三(已更新)", "zhangsan_new@example.com").await.unwrap();
    println!("更新用户{}", id1);

    println!("\n=== 事务操作 ===");
    let author = create_user_with_articles(&pool, "赵六", "zhaoliu@example.com", vec![
        ("Rust 入门", "Rust 是系统编程语言..."),
        ("Tokio 异步", "Tokio 是异步运行时..."),
        ("SQLx 实战", "SQLx 提供类型安全SQL..."),
    ]).await.unwrap();
    println!("事务创建: 用户{}及3篇文章", author);

    println!("\n=== 高级查询 ===");
    let articles = get_user_articles(&pool).await.unwrap();
    for a in &articles { println!("  [{}] {} by {}", a.article_id, a.title, a.author_name); }

    let found = search_users(&pool, "张").await.unwrap();
    println!("搜索'张': {}个结果", found.len());

    let counts = count_articles_per_user(&pool).await.unwrap();
    for (name, cnt) in &counts { println!("  {} -> {}篇", name, cnt); }

    println!("\n=== 删除 ===");
    println!("删除用户{}: {}", id2, delete_user(&pool, id2).await.unwrap());

    let all = list_users(&pool, 1, 100).await.unwrap();
    println!("最终用户({}人): {:?}", all.len(), all.iter().map(|u| &u.name).collect::<Vec<_>>());
    info!("MySQL CRUD Demo 完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 尝试连接数据库，无法连接则跳过测试
    async fn get_test_pool() -> Option<MySqlPool> {
        let url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/rust_study".to_string());
        create_pool(&url).await.ok()
    }

    macro_rules! skip_if_no_db {
        ($pool:expr) => {
            match $pool {
                Some(p) => p,
                None => { eprintln!("跳过: MySQL 不可用"); return; }
            }
        };
    }

    #[tokio::test]
    async fn test_init_tables() {
        let pool = skip_if_no_db!(get_test_pool().await);
        let result = init_tables(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_insert_and_get_user() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let email = format!("test_insert_{}@test.com", uuid::Uuid::new_v4());
        let id = insert_user(&pool, "测试用户", &email, Some(25)).await.unwrap();
        assert!(id > 0);

        let user = get_user_by_id(&pool, id).await.unwrap();
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, email);
        assert_eq!(user.age, Some(25));

        // 清理
        delete_user(&pool, id).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let email = format!("test_update_{}@test.com", uuid::Uuid::new_v4());
        let id = insert_user(&pool, "原始名", &email, None).await.unwrap();

        let new_email = format!("updated_{}@test.com", uuid::Uuid::new_v4());
        let ok = update_user(&pool, id, "新名字", &new_email).await.unwrap();
        assert!(ok);

        let user = get_user_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(user.name, "新名字");

        delete_user(&pool, id).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_nonexistent() {
        let pool = skip_if_no_db!(get_test_pool().await);
        let ok = update_user(&pool, 999999, "x", "x@x.com").await.unwrap();
        assert!(!ok);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let email = format!("test_del_{}@test.com", uuid::Uuid::new_v4());
        let id = insert_user(&pool, "待删除", &email, None).await.unwrap();
        assert!(delete_user(&pool, id).await.unwrap());
        assert!(!delete_user(&pool, id).await.unwrap()); // 第二次删除失败

        let user = get_user_by_id(&pool, id).await.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_list_users_pagination() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let mut ids = Vec::new();
        for i in 0..3 {
            let email = format!("test_page_{}_{}@test.com", i, uuid::Uuid::new_v4());
            ids.push(insert_user(&pool, &format!("分页用户{}", i), &email, None).await.unwrap());
        }

        let page1 = list_users(&pool, 1, 2).await.unwrap();
        assert!(page1.len() <= 2);

        for id in ids { delete_user(&pool, id).await.unwrap(); }
    }

    #[tokio::test]
    async fn test_transaction_user_with_articles() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let email = format!("test_tx_{}@test.com", uuid::Uuid::new_v4());
        let user_id = create_user_with_articles(
            &pool, "事务用户", &email,
            vec![("文章A", "内容A"), ("文章B", "内容B")],
        ).await.unwrap();

        let articles = get_user_articles(&pool).await.unwrap();
        let user_articles: Vec<_> = articles.iter().filter(|a| a.author_name == "事务用户").collect();
        assert_eq!(user_articles.len(), 2);

        // 级联删除
        delete_user(&pool, user_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_search_users() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let email = format!("search_test_{}@test.com", uuid::Uuid::new_v4());
        let id = insert_user(&pool, "搜索目标XYZ", &email, None).await.unwrap();

        let results = search_users(&pool, "XYZ").await.unwrap();
        assert!(results.iter().any(|u| u.name.contains("XYZ")));

        delete_user(&pool, id).await.unwrap();
    }

    #[tokio::test]
    async fn test_count_articles_per_user() {
        let pool = skip_if_no_db!(get_test_pool().await);
        init_tables(&pool).await.unwrap();

        let counts = count_articles_per_user(&pool).await;
        assert!(counts.is_ok());
    }
}
