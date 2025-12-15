//! 条件语句：if-else
//!
//! Rust 中的 if 表达式用于根据条件执行不同的代码分支。
//! if 表达式可以返回值，每个分支必须返回相同类型的值。

pub fn main() {
    println!("=== if-else 条件语句 ===");

    // 1. 基本的 if 语句
    let number = 7;

    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // 2. 多个 else if 分支
    let number = 6;

    if number % 4 == 0 {
        println!("能被 4 整除");
    } else if number % 3 == 0 {
        println!("能被 3 整除");
    } else if number % 2 == 0 {
        println!("能被 2 整除");
    } else {
        println!("不能被 2、3、4 整除");
    }

    // 3. 在 let 语句中使用 if（if 表达式）
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if 表达式的值: {}", number);

    // 注意：if 的每个分支必须返回相同类型的值
    // let number = if condition { 5 } else { "six" }; // 错误：类型不匹配

    // 4. 嵌套的 if 语句
    let x = 15;
    let y = 10;

    if x > 10 {
        if y > 5 {
            println!("x > 10 且 y > 5");
        } else {
            println!("x > 10 但 y <= 5");
        }
    } else {
        println!("x <= 10");
    }

    // 5. 使用 match 替代复杂的 if-else（更清晰）
    let number = 42;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("质数"),
        13..=19 => println!("十几"),
        _ => println!("其他数字"),
    }

    // 6. 条件表达式中的逻辑运算符
    let a = true;
    let b = false;
    let c = true;

    if a && b {
        println!("a 和 b 都为真");
    } else if a || b {
        println!("a 或 b 为真");
    } else if !c {
        println!("c 为假");
    }

    // 7. 比较运算符
    let x = 5;
    let y = 10;

    if x == y {
        println!("x 等于 y");
    } else if x != y {
        println!("x 不等于 y");
    } else if x < y {
        println!("x 小于 y");
    } else if x > y {
        println!("x 大于 y");
    } else if x <= y {
        println!("x 小于等于 y");
    } else if x >= y {
        println!("x 大于等于 y");
    }

    // 8. 复杂的条件判断
    let age = 25;
    let has_permission = true;

    if age >= 18 && has_permission {
        println!("可以访问");
    } else if age < 18 {
        println!("年龄不足");
    } else {
        println!("没有权限");
    }
}