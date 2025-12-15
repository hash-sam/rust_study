//! 流程控制模块
//!
//! 包含 if-else、循环等流程控制示例。

// 声明子模块
pub mod if_else;
pub mod loops;

pub fn run_all() {
    println!("\n--- if-else 示例 ---");
    if_else::main();

    println!("\n--- 循环示例 ---");
    loops::main();
}