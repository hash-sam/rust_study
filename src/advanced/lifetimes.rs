//! 生命周期（Lifetimes）
//!
//! 生命周期是 Rust 中引用有效的作用域。
//! 生命周期注解描述了多个引用的生命周期如何相互关联。
//! 生命周期的主要目标是避免悬垂引用。

pub fn main() {
    println!("=== 生命周期基础 ===");

    // 1. 基本生命周期示例
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);

    // 2. 结构体中的生命周期
    let novel = String::from("从前有座山。山里有座庙...");
    let first_sentence = novel.split('。').next().expect("找不到句号。");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("重要摘录: {}", i.part);

    // 3. 生命周期省略规则
    let s = String::from("hello");
    let len = first_word(&s);
    println!("第一个单词长度: {}", len);

    // 4. 方法中的生命周期
    let excerpt = ImportantExcerpt { part: "hello" };
    let announcement = excerpt.announce_and_return_part("重要通知");
    println!("通知: {}", announcement);

    // 5. 静态生命周期
    let s: &'static str = "我是一个静态字符串";
    println!("静态字符串: {}", s);

    // 6. 生命周期和泛型结合
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "今天是个好日子!"
    );
    println!("带通知的最长字符串: {}", result);

    // 7. 生命周期在 trait 对象中
    let trait_object: Box<dyn Printable> = Box::new(String::from("hello"));
    trait_object.print();

    // 8. 生命周期约束
    let wrapper = Wrapper { value: "hello" };
    println!("包装器值: {}", wrapper.value);

    // 9. 高阶 trait 边界（HRTB）
    let numbers = vec![1, 2, 3];
    let sum = sum_refs_simple(&numbers);
    println!("引用之和: {}", sum);

    // 10. 生命周期子类型
    let static_str = "静态字符串";
    let longer;
    {
        let dynamic_str = String::from("动态字符串");
        longer = longest_lifetime(static_str, dynamic_str.as_str());
        println!("更长的生命周期: {}", longer); // 在 dynamic_str 作用域内使用
    }
    // println!("更长的生命周期: {}", longer); // 错误：longer 的生命周期不够长
}

// 生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期省略规则示例
// 规则1：每个引用参数都有自己的生命周期
// 规则2：如果只有一个输入生命周期，它被赋予所有输出生命周期
// 规则3：如果有多个输入生命周期，但其中一个是 &self 或 &mut self，self 的生命周期被赋予所有输出生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 方法中的生命周期
impl<'a> ImportantExcerpt<'a> {
    // 规则3应用：返回值的生命周期与 self 相同
    fn level(&self) -> i32 {
        3
    }

    // 显式生命周期注解
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意! {}", announcement);
        self.part
    }
}

// 生命周期和泛型结合
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// trait 对象中的生命周期
trait Printable {
    fn print(&self);
}

impl Printable for String {
    fn print(&self) {
        println!("字符串: {}", self);
    }
}

// 生命周期约束
struct Wrapper<'a, T: 'a + ?Sized> {
    value: &'a T,
}

impl<'a, T> Wrapper<'a, T> {
    fn get_value(&self) -> &'a T {
        self.value
    }
}

// 高阶 trait 边界（Higher-Rank Trait Bounds）
// 简化示例，避免复杂类型约束
fn sum_refs_simple(items: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in items {
        sum = sum + item;
    }
    sum
}

// 生命周期子类型
fn longest_lifetime<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 复杂生命周期示例
struct Context<'a>(&'a str);

struct Parser<'a, 'c: 'a> {
    context: &'a Context<'c>,
}

impl<'a, 'c: 'a> Parser<'a, 'c> {
    fn parse(&self) -> Result<(), &'c str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// 生命周期和闭包
fn make_adder<'a>(x: &'a i32) -> Box<dyn Fn(i32) -> i32 + 'a> {
    Box::new(move |y| x + y)
}

// 生命周期和迭代器
struct Iter<'a, T> {
    data: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// 生命周期在关联类型中
trait Process {
    type Item<'a> where Self: 'a;

    fn process<'a>(&'a self) -> Self::Item<'a>;
}

struct TextProcessor {
    text: String,
}

impl Process for TextProcessor {
    type Item<'a> = &'a str where Self: 'a;

    fn process<'a>(&'a self) -> Self::Item<'a> {
        &self.text
    }
}

// 生命周期和 unsafe 代码
unsafe fn raw_pointers_example() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 注意：解引用原始指针需要 unsafe 块
    // println!("r1: {}", *r1);
    // println!("r2: {}", *r2);
}

// 生命周期标记（PhantomData）
use std::marker::PhantomData;

struct Marker<'a, T> {
    data: PhantomData<&'a T>,
}

impl<'a, T> Marker<'a, T> {
    fn new() -> Self {
        Marker {
            data: PhantomData,
        }
    }
}

// 生命周期和并发
use std::thread;

fn spawn_thread() {
    let s = String::from("hello");

    // 错误：s 的生命周期不够长
    // thread::spawn(|| {
    //     println!("{}", s);
    // });

    // 正确：移动所有权
    thread::spawn(move || {
        println!("{}", s);
    });
}

// 生命周期诊断技巧
fn lifetime_diagnostics() {
    // 常见错误：返回局部变量的引用
    // fn dangling() -> &String {
    //     let s = String::from("hello");
    //     &s
    // } // s 在这里被丢弃，返回悬垂引用

    // 解决方案：返回 String 而不是引用
    fn not_dangling() -> String {
        let s = String::from("hello");
        s
    }
}

// 生命周期和模式匹配
fn pattern_matching_lifetime<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// 在 main 中测试
// fn main() {
//     // 测试 parse_context
//     let context = Context("hello");
//     match parse_context(context) {
//         Ok(_) => println!("解析成功"),
//         Err(e) => println!("解析失败: {}", e),
//     }
//
//     // 测试 make_adder
//     let x = 5;
//     let add_five = make_adder(&x);
//     println!("5 + 3 = {}", add_five(3));
//
//     // 测试 Iter
//     let data = vec![1, 2, 3, 4, 5];
//     let iter = Iter { data: &data, index: 0 };
//     for item in iter {
//         println!("迭代器项: {}", item);
//     }
//
//     // 测试 TextProcessor
//     let processor = TextProcessor {
//         text: String::from("hello world"),
//     };
//     let processed = processor.process();
//     println!("处理后的文本: {}", processed);
//
//     // 测试 pattern_matching_lifetime
//     let s = "hello world";
//     let first = pattern_matching_lifetime(s);
//     println!("第一个单词: {}", first);
// }

// 生命周期最佳实践
// 1. 尽可能让编译器推断生命周期
// 2. 只在必要时添加生命周期注解
// 3. 使用有意义的生命周期名称（如 'a, 'b, 'ctx, 'iter）
// 4. 理解生命周期省略规则
// 5. 使用生命周期来确保内存安全

// 常见生命周期模式
// 1. 输入生命周期：函数参数中的引用
// 2. 输出生命周期：函数返回值中的引用
// 3. 结构体生命周期：结构体字段中的引用
// 4. 方法生命周期：方法中的 self 引用
// 5. trait 对象生命周期：dyn Trait + 'a