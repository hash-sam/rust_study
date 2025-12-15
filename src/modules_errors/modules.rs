//! 模块系统
//!
//! Rust 的模块系统包括：
//! - 包（Packages）：Cargo 功能，允许构建、测试和分享 crate
//! - Crate：一个树形模块结构，形成库或二进制项目
//! - 模块（Modules）和 use：控制作用域和路径的私有性
//! - 路径（Path）：命名项（如结构体、函数、模块）的方式

// 声明子模块
mod front_of_house {
    // 模块可以包含其他模块
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }

        fn seat_at_table() {
            println!("安排座位");
        }
    }

    mod serving {
        fn take_order() {
            println!("接受点餐");
        }

        fn serve_order() {
            println!("上菜");
        }

        fn take_payment() {
            println!("收款");
        }
    }
}

// 使用绝对路径（注释掉，因为 front_of_house 在当前模块内）
// use crate::front_of_house::hosting; // 错误：front_of_house 不在 crate 根

// 使用相对路径
use self::front_of_house::hosting;
// 或者
// use front_of_house::hosting;

fn deliver_order() {
    println!("送餐");
}

mod back_of_house {
    // 结构体字段默认是私有的
    pub struct Breakfast {
        pub toast: String,      // 公共字段
        seasonal_fruit: String, // 私有字段
    }

    impl Breakfast {
        // 关联函数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举的变体默认都是公共的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // 私有函数
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 使用 super 访问父模块
    }

    fn cook_order() {
        println!("烹饪");
    }
}

// 使用 pub use 重导出
pub use self::front_of_house::hosting as pub_hosting;

// 嵌套路径简化 use 语句
use std::{cmp::Ordering, io};
// 等价于：
// use std::cmp::Ordering;
// use std::io;

// 使用 self（注释掉，因为上面已经导入了 io）
// use std::io::{self, Write}; // 重复导入，注释掉
// 等价于：
// use std::io;
// use std::io::Write;

// 通配符导入（谨慎使用）
// use std::collections::*;

pub fn main() {
    println!("=== 模块系统示例 ===");

    // 1. 使用模块中的函数
    hosting::add_to_waitlist();
    // hosting::seat_at_table(); // 错误：私有函数

    // 2. 使用结构体
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("我要 {} 吐司", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // 错误：私有字段

    // 3. 使用枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 4. 使用重导出的模块
    pub_hosting::add_to_waitlist();

    // 5. 标准库模块使用示例
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败");

    match guess.trim().parse::<i32>() {
        Ok(num) => match num.cmp(&42) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => println!("正确!"),
        },
        Err(_) => println!("请输入数字!"),
    }

    // 6. 模块组织最佳实践
    // - 相关功能组织在一起
    // - 减少耦合
    // - 提高可重用性
    // - 控制可见性

    // 7. 文件系统模块
    // 当模块变大时，可以拆分成单独的文件
    // mod garden; // 这会查找 src/garden.rs 或 src/garden/mod.rs
}

// 示例：模块可见性规则
mod visibility_example {
    mod outer_module {
        pub mod inner_module {
            pub fn public_function() {
                println!("公共函数");
            }

            fn private_function() {
                println!("私有函数");
            }

            pub fn indirect_access() {
                print!("间接访问: ");
                private_function();
            }
        }

        pub fn call_public_function() {
            print!("从 outer_module 调用: ");
            inner_module::public_function();
            print!("从 outer_module 间接调用: ");
            inner_module::indirect_access();
        }
    }

    pub fn test_visibility() {
        println!("\n=== 可见性测试 ===");

        // 可以访问 outer_module 的公共函数
        outer_module::call_public_function();

        // 可以直接访问 inner_module 的公共函数
        outer_module::inner_module::public_function();

        // 不能访问私有函数
        // outer_module::inner_module::private_function(); // 错误
    }
}

// 使用 visibility_example
// visibility_example::test_visibility(); // 需要在 main 中调用

// 示例：模块作为接口
mod shapes {
    // 公共 trait
    pub trait Area {
        fn area(&self) -> f64;
    }

    // 公共结构体
    pub struct Circle {
        radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // 私有结构体（模块内部使用）
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
    }

    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    // 公共函数返回实现 Area 的类型
    pub fn create_circle(radius: f64) -> impl Area {
        Circle::new(radius)
    }

    // 私有辅助函数
    fn validate_positive(value: f64) -> bool {
        value > 0.0
    }
}

// 示例：使用 as 关键字重命名导入
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn function1() -> FmtResult {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// 示例：模块文档注释
/// 这个模块包含数学工具函数
pub mod math {
    /// 计算两个数的和
    ///
    /// # 示例
    ///
    /// ```
    /// use my_crate::math::add;
    /// assert_eq!(add(2, 3), 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// 计算两个数的乘积
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // 私有函数，模块内部使用
    fn internal_helper() {
        // 实现细节
    }
}

// 在 main 函数中测试
// fn main() {
//     visibility_example::test_visibility();
//
//     let circle = shapes::create_circle(5.0);
//     println!("圆面积: {}", circle.area());
//
//     println!("2 + 3 = {}", math::add(2, 3));
//     println!("2 * 3 = {}", math::multiply(2, 3));
// }