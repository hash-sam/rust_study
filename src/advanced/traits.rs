//! Trait
//!
//! Trait 定义了类型的行为（方法）。类似于其他语言中的接口（interface）。
//! Trait 可以用于：
//! 1. 定义共享的行为
//! 2. 作为泛型约束（trait bounds）
//! 3. 实现多态

use std::fmt;

pub fn main() {
    println!("=== Trait 基础 ===");

    // 1. 基本 Trait 使用
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 条新推文: {}", tweet.summarize());
    println!("默认摘要: {}", tweet.default_summary());

    // 2. Trait 作为参数
    notify(&tweet);
    notify_with_impl(&tweet);

    // 3. Trait 作为返回值
    let article = returns_summarizable();
    println!("返回的文章: {}", article.summarize());

    // 4. 使用 Trait 实现运算符重载
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("点相加: {:?}", p3);

    // 5. 使用 Trait 实现显示格式化
    let point = Point { x: 5, y: 10 };
    println!("点显示: {}", point);
    println!("点调试: {:?}", point);

    // 6. 派生 Trait
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect1 能容纳 rect2 吗? {}", rect1.can_hold(&rect2));
    println!("rect1 能容纳 rect3 吗? {}", rect1.can_hold(&rect3));

    // 7. Trait 对象（动态分发）
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("确定"),
            }),
            Box::new(TextField {
                width: 75,
                height: 10,
                placeholder: String::from("请输入..."),
            }),
        ],
    };
    screen.run();

    // 8. Trait 继承
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    person.greet();
    person.describe();

    // 9. 关联类型
    let counter = Counter { value: 0 };
    let doubled = counter.map(|x| x * 2);
    println!("计数器加倍: {}", doubled.value);

    // 10. 默认泛型类型参数
    let meters1 = Meters(5.0);
    let meters2 = Meters(3.0);
    let total = meters1 + meters2;
    println!("总距离: {:?}", total);

    // 11. 完全限定语法
    let person = Human;
    person.fly(); // 调用 Human 的 fly
    Pilot::fly(&person); // 调用 Pilot 的 fly
    Wizard::fly(&person); // 调用 Wizard 的 fly

    // 12. 父 Trait
    let outline_point = OutlinePoint { x: 1, y: 3 };
    println!("轮廓点: {}", outline_point);

    // 13. newtype 模式
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("包装器: {}", w);
}

// 基本 Trait 定义
pub trait Summary {
    // 方法签名
    fn summarize(&self) -> String;

    // 默认实现
    fn default_summary(&self) -> String {
        String::from("(阅读更多...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("突发新闻! {}", item.summarize());
}

// Trait bound 语法
pub fn notify_with_impl<T: Summary>(item: &T) {
    println!("通知: {}", item.summarize());
}

// 返回实现 Trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 运算符重载
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 显示格式化
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 派生 Trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Trait 对象（动态分发）
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: {}x{}, 标签: {}", self.width, self.height, self.label);
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("绘制文本框: {}x{}, 占位符: {}", self.width, self.height, self.placeholder);
    }
}

// Trait 继承
trait Greet {
    fn greet(&self);
}

trait Describe: Greet {
    fn describe(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Greet for Person {
    fn greet(&self) {
        println!("你好，我是{}!", self.name);
    }
}

impl Describe for Person {
    fn describe(&self) {
        println!("我叫{}，今年{}岁。", self.name, self.age);
    }
}

// 关联类型
trait Map {
    type Output;
    fn map<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(i32) -> i32;
}

struct Counter {
    value: i32,
}

impl Map for Counter {
    type Output = Counter;

    fn map<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(i32) -> i32,
    {
        Counter {
            value: f(self.value),
        }
    }
}

// 默认泛型类型参数
#[derive(Debug)]
struct Meters(f64);

impl std::ops::Add for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

// 完全限定语法
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("挥动手臂");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("这是你的机长在说话");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("起飞!");
    }
}

// 父 Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

struct OutlinePoint {
    x: i32,
    y: i32,
}

impl fmt::Display for OutlinePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for OutlinePoint {}

// newtype 模式
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 其他有用的 Trait 示例

// Clone 和 Copy
#[derive(Clone, Copy)]
struct Coordinates {
    x: f64,
    y: f64,
}

// PartialEq 和 Eq
#[derive(PartialEq, Eq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// PartialOrd 和 Ord
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Priority(u8);

// Hash
#[derive(Hash)]
struct UserId(u64);

// Default
#[derive(Default)]
struct Config {
    timeout: u32,
    retries: u8,
}

// From 和 Into
struct Inches(i32);
struct Centimeters(i32);

impl From<Inches> for Centimeters {
    fn from(inches: Inches) -> Self {
        Centimeters((inches.0 as f64 * 2.54) as i32)
    }
}

// TryFrom 和 TryInto
struct PositiveNumber(i32);

impl TryFrom<i32> for PositiveNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(PositiveNumber(value))
        } else {
            Err("必须是正数".to_string())
        }
    }
}

// Deref 和 DerefMut
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Drop
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("释放资源: {}", self.name);
    }
}

// Iterator
struct Countdown {
    count: i32,
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let current = self.count;
            self.count -= 1;
            Some(current)
        } else {
            None
        }
    }
}

// 在 main 中测试
// fn main() {
//     // 测试 From/Into
//     let inches = Inches(10);
//     let cm: Centimeters = inches.into();
//     println!("10 英寸 = {} 厘米", cm.0);
//
//     // 测试 TryFrom/TryInto
//     let positive: Result<PositiveNumber, _> = 5.try_into();
//     match positive {
//         Ok(num) => println!("正数: {}", num.0),
//         Err(e) => println!("错误: {}", e),
//     }
//
//     // 测试 Deref
//     let x = 5;
//     let y = MyBox(x);
//     assert_eq!(5, *y);
//
//     // 测试 Drop
//     {
//         let _resource = Resource {
//             name: "文件句柄".to_string(),
//         };
//         println!("资源在作用域内");
//     }
//     println!("资源已离开作用域");
//
//     // 测试 Iterator
//     let countdown = Countdown { count: 3 };
//     for num in countdown {
//         println!("倒计时: {}", num);
//     }
// }