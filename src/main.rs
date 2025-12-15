//! Rust 学习项目 - 主文件
//!
//! 这个项目包含了 Rust 的各种语法和功能的示例。
//! 按主题组织在子目录中。

// 导入各个模块
mod basics;
mod control_flow;
mod ownership;
mod structs_enums;
mod collections;
mod modules_errors;
mod advanced;

use std::io;

fn main() {
    println!("=== Rust 学习项目 ===");
    println!("选择要运行的示例：");
    println!("1. 基础语法（变量、数据类型、函数）");
    println!("2. 流程控制（if-else、循环）");
    println!("3. 所有权和借用");
    println!("4. 结构体、枚举和模式匹配");
    println!("5. 集合（向量、字符串、哈希映射）");
    println!("6. 模块和错误处理");
    println!("7. 高级特性（泛型、Trait、生命周期）");
    println!("8. 运行所有示例");
    println!("0. 退出");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("读取输入失败");

    match choice.trim() {
        "1" => run_basics(),
        "2" => run_control_flow(),
        "3" => run_ownership(),
        "4" => run_structs_enums(),
        "5" => run_collections(),
        "6" => run_modules_errors(),
        "7" => run_advanced(),
        "8" => run_all(),
        "0" => println!("再见！"),
        _ => println!("无效选择"),
    }
}

fn run_basics() {
    println!("\n=== 运行基础语法示例 ===");
    basics::run_all();
}

fn run_control_flow() {
    println!("\n=== 运行流程控制示例 ===");
    control_flow::run_all();
}

fn run_ownership() {
    println!("\n=== 运行所有权示例 ===");
    ownership::run_all();
}

fn run_structs_enums() {
    println!("\n=== 运行结构体和枚举示例 ===");
    structs_enums::run_all();
}

fn run_collections() {
    println!("\n=== 运行集合示例 ===");
    collections::run_all();
}

fn run_modules_errors() {
    println!("\n=== 运行模块和错误处理示例 ===");
    modules_errors::run_all();
}

fn run_advanced() {
    println!("\n=== 运行高级特性示例 ===");
    advanced::run_all();
}

fn run_all() {
    run_basics();
    run_control_flow();
    run_ownership();
    run_structs_enums();
    run_collections();
    run_modules_errors();
    run_advanced();
    println!("\n=== 所有示例运行完成 ===");
}
