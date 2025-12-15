//! 高级特性模块
//!
//! 包含泛型、Trait、生命周期等高级特性示例。

// 声明子模块
pub mod generics;
pub mod traits;
pub mod lifetimes;

pub fn run_all() {
    println!("\n--- 泛型示例 ---");
    generics::main();

    println!("\n--- Trait 示例 ---");
    traits::main();

    println!("\n--- 生命周期示例 ---");
    lifetimes::main();
}