//! 变量和常量
//!
//! Rust 中的变量默认是不可变的（immutable），使用 `let` 关键字声明。
//! 使用 `mut` 关键字可以使变量可变（mutable）。
//! 常量使用 `const` 关键字声明，必须在编译时确定值。

pub fn main() {
    // 1. 不可变变量
    let x = 5;
    println!("不可变变量 x = {}", x);
    // x = 6; // 错误：不能给不可变变量重新赋值

    // 2. 可变变量
    let mut y = 5;
    println!("可变变量 y = {}", y);
    y = 6;
    println!("修改后 y = {}", y);

    // 3. 变量遮蔽（shadowing）
    let z = 5;
    let z = z + 1; // 创建新的变量 z，遮蔽了之前的 z
    {
        let z = z * 2; // 在作用域内再次遮蔽
        println!("内部作用域中的 z = {}", z); // 12
    }
    println!("外部作用域中的 z = {}", z); // 6

    // 4. 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 5. 变量类型注解
    let guess: u32 = "42".parse().expect("不是一个数字!");
    println!("带类型注解的变量 guess = {}", guess);

    // 6. 未使用的变量（使用下划线前缀避免警告）
    let _unused_variable = 10;

    // 7. 变量解构
    let (a, b) = (1, 2);
    println!("解构变量: a = {}, b = {}", a, b);

    // 8. 变量作用域
    {
        let inner_var = "内部变量";
        println!("{}", inner_var);
    }
    // println!("{}", inner_var); // 错误：inner_var 已离开作用域
}