//! 基础语法模块
//!
//! 包含变量、数据类型、函数等基础语法示例。

// 声明子模块
pub mod variables;
pub mod data_types;
pub mod functions;

pub fn run_all() {
    println!("\n--- 变量示例 ---");
    variables::main();

    println!("\n--- 数据类型示例 ---");
    data_types::main();

    println!("\n--- 函数示例 ---");
    functions::main();
}