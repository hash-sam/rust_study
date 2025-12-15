//! 模式匹配（Pattern Matching）
//!
//! Rust 的模式匹配非常强大，可以用于解构各种数据类型。
//! match 表达式是 Rust 中最强大的控制流运算符之一。

pub fn main() {
    println!("=== 模式匹配基础 ===");

    // 1. 基本 match 表达式
    let number = 13;
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("质数"),
        13..=19 => println!("十几"),
        _ => println!("其他数字"),
    }

    // 2. match 返回值
    let number = 5;
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "其他",
    };
    println!("数字 {} 是 {}", number, description);

    // 3. 匹配枚举
    let coin = Coin::Quarter(UsState::California);
    let value = value_in_cents(&coin);
    println!("硬币价值: {} 美分", value);

    // 4. 匹配 Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("加一: {:?}, {:?}", six, none);

    // 5. 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => println!("前进 3 步"),
        7 => println!("后退 7 步"),
        other => println!("前进 {} 步", other), // 捕获所有其他值
    }

    // 6. 匹配范围模式
    let x = 5;
    match x {
        1..=5 => println!("一到五"),
        6..=10 => println!("六到十"),
        _ => println!("其他"),
    }

    // 7. 解构结构体
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x, y } => println!("在 ({}, {})", x, y),
    }

    // 8. 解构枚举
    let msg = Message::ChangeColor(255, 0, 0);
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("文本消息: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: RGB({}, {}, {})", r, g, b),
    }

    // 9. 解构嵌套结构
    // let shape = Shape::Circle(Point { x: 0.0, y: 0.0 }, 10.0); // 类型不匹配，注释掉
    // match shape {
    //     Shape::Circle(center, radius) => println!("圆心: ({}, {}), 半径: {}", center.x, center.y, radius),
    //     Shape::Rectangle(p1, p2) => println!("矩形: ({}, {}) 到 ({}, {})", p1.x, p1.y, p2.x, p2.y),
    //     Shape::Triangle(p1, p2, p3) => println!("三角形: 三个点"),
    // }

    // 10. 匹配守卫（match guards）
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 11. @ 绑定
    let msg = MessageWithId::Hello { id: 5 };
    match msg {
        MessageWithId::Hello { id: id_variable @ 3..=7 } => {
            println!("在范围 3-7 内找到 id: {}", id_variable)
        }
        MessageWithId::Hello { id: 10..=12 } => {
            println!("在另一个范围 10-12 内找到 id")
        }
        MessageWithId::Hello { id } => println!("找到其他 id: {}", id),
    }

    // 12. if let 语法糖
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用最喜欢的颜色: {}", color);
    } else if is_tuesday {
        println!("星期二是绿色日!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色");
        } else {
            println!("使用橙色");
        }
    } else {
        println!("使用蓝色");
    }

    // 13. while let 循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 14. for 循环中的模式匹配
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} 在索引 {}", value, index);
    }

    // 15. let 语句中的模式匹配
    let (x, y, z) = (1, 2, 3);
    println!("x={}, y={}, z={}", x, y, z);

    // 16. 函数参数中的模式匹配
    let point = (3, 5);
    print_coordinates(&point);
}

// 枚举定义
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的硬币", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 结构体定义
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point),
    Triangle(Point, Point, Point),
}

enum MessageWithId {
    Hello { id: i32 },
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("当前位置: ({}, {})", x, y);
}

// 忽略值的模式
fn ignore_patterns() {
    let numbers = (2, 4, 8, 16, 32);

    // 忽略部分值
    match numbers {
        (first, _, third, _, fifth) => {
            println!("一些数字: {}, {}, {}", first, third, fifth)
        }
    }

    // 忽略剩余值
    match numbers {
        (first, ..) => {
            println!("第一个数字是: {}", first)
        }
    }

    // 忽略单个值
    let _x = 5; // 使用下划线忽略未使用的变量
    let y = 10; // 会产生警告
}

// 匹配引用和解引用
fn match_references() {
    let reference = &4;

    match reference {
        &val => println!("通过解引用得到: {}", val),
    }

    // 更好的方式：使用 ref 模式
    match *reference {
        val => println!("直接解引用: {}", val),
    }
}