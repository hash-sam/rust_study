//! 所有权模块
//!
//! 包含所有权、引用和借用等示例。

// 声明子模块
pub mod ownership_basics;
pub mod references_borrowing;

pub fn run_all() {
    println!("\n--- 所有权基础示例 ---");
    ownership_basics::main();

    println!("\n--- 引用和借用示例 ---");
    references_borrowing::main();
}