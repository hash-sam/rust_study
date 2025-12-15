//! 结构体和枚举模块
//!
//! 包含结构体、枚举和模式匹配等示例。

// 声明子模块
pub mod structs;
pub mod enums;
pub mod pattern_matching;

pub fn run_all() {
    println!("\n--- 结构体示例 ---");
    structs::main();

    println!("\n--- 枚举示例 ---");
    enums::main();

    println!("\n--- 模式匹配示例 ---");
    pattern_matching::main();
}