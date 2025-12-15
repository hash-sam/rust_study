//! 集合模块
//!
//! 包含向量、字符串、哈希映射等集合类型示例。

// 声明子模块
pub mod vectors;
pub mod strings;
pub mod hashmaps;

pub fn run_all() {
    println!("\n--- 向量示例 ---");
    vectors::main();

    println!("\n--- 字符串示例 ---");
    strings::main();

    println!("\n--- 哈希映射示例 ---");
    hashmaps::main();
}