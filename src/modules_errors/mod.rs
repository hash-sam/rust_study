//! 模块和错误处理模块
//!
//! 包含模块系统和错误处理示例。

// 声明子模块
pub mod modules;
pub mod error_handling;

pub fn run_all() {
    println!("\n--- 模块系统示例 ---");
    modules::main();

    println!("\n--- 错误处理示例 ---");
    error_handling::main();
}