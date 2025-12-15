//! 枚举（Enums）
//!
//! 枚举允许你定义一个类型，该类型可以是多个变体之一。
//! Rust 的枚举非常强大，每个变体可以关联不同类型和数量的数据。

pub fn main() {
    println!("=== 枚举基础 ===");

    // 1. 基本枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("IP 类型: {:?} 和 {:?}", four, six);

    // 2. 将数据与枚举变体关联
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IP 地址: {:?} 和 {:?}", home, loopback);

    // 3. 枚举变体可以关联不同类型的数据
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // 4. 枚举方法
    msg1.call();
    msg2.call();

    // 5. Option 枚举（Rust 没有 null）
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("Option 值: {:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // 6. 使用 match 处理 Option
    let x: Option<i32> = Some(5);
    match x {
        Some(i) => println!("有值: {}", i),
        None => println!("没有值"),
    }

    // 7. Result 枚举（用于错误处理）
    let result: Result<i32, String> = Ok(42);
    let error: Result<i32, String> = Err(String::from("出错了"));

    match result {
        Ok(value) => println!("成功: {}", value),
        Err(e) => println!("错误: {}", e),
    }

    // 8. 复杂枚举
    let shape = Shape::Circle(Point { x: 0.0, y: 0.0 }, 10.0);
    let area = shape.area();
    println!("形状面积: {}", area);

    // 9. 枚举中的模式匹配
    let coin = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(&coin);
    println!("硬币价值: {} 美分", value);

    // 10. if let 语法糖
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值是: {}", max);
    }

    // 11. while let 循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // 12. 枚举作为函数参数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

// 基本枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 枚举变体关联数据
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// 更复杂的枚举
enum Message {
    Quit, // 没有关联数据
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String), // 单个 String
    ChangeColor(i32, i32, i32), // 三个 i32
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        // 方法实现
        match self {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色到 RGB({}, {}, {})", r, g, b),
        }
    }
}

// 复杂枚举示例
struct Point {
    x: f64,
    y: f64,
}

enum Shape {
    Circle(Point, f64), // 圆心和半径
    Rectangle(Point, Point), // 两个对角点
    Triangle(Point, Point, Point), // 三个顶点
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(_, radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(p1, p2) => {
                let width = (p2.x - p1.x).abs();
                let height = (p2.y - p1.y).abs();
                width * height
            }
            Shape::Triangle(p1, p2, p3) => {
                // 使用海伦公式计算三角形面积
                let a = distance(p1, p2);
                let b = distance(p2, p3);
                let c = distance(p3, p1);
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

// 枚举和模式匹配示例
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运硬币!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的 25 美分硬币", state);
            25
        }
    }
}

// 枚举作为函数参数
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由 IPv4"),
        IpAddrKind::V6 => println!("路由 IPv6"),
    }
}

// 枚举中的泛型（后续会详细讲解）
// 注意：这里注释掉，因为会与标准库的 Option 和 Result 冲突
// enum MyOption<T> {
//     Some(T),
//     None,
// }
//
// enum MyResult<T, E> {
//     Ok(T),
//     Err(E),
// }