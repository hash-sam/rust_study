//! 泛型（Generics）
//!
//! 泛型允许编写可以处理多种类型的代码，提高代码复用性。
//! Rust 在编译时进行泛型代码的单态化（monomorphization），
//! 为每个具体类型生成特定代码，保证运行时零成本抽象。

pub fn main() {
    println!("=== 泛型基础 ===");

    // 1. 泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);

    // 2. 泛型结构体
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);

    // 3. 多个泛型参数
    let point = Point2 { x: 5, y: 10.4 };
    println!("混合类型点: {:?}", point);

    // 4. 泛型枚举（标准库中的 Option 和 Result）
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    println!("Option 示例: {:?}, {:?}", some_number, some_string);

    // 5. 泛型方法
    let p = Point { x: 5, y: 10 };
    println!("x 坐标: {}", p.x());
    // println!("到原点的距离: {}", p.distance_from_origin()); // 只有 f32 Point 有这个方法

    let p_float = Point { x: 5.0, y: 10.0 };
    println!("浮点数点到原点的距离: {}", p_float.distance_from_origin());

    // 6. 使用 trait bounds 约束泛型
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 条新推文: {}", summarize(&tweet));

    // 7. 使用 where 子句简化 trait bounds
    let result = compare_and_print(&5, &10);
    println!("比较结果: {}", result);

    // 8. 泛型与生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);

    // 9. 泛型性能
    // Rust 在编译时进行单态化，为每个具体类型生成特定代码
    // 这意味着泛型没有运行时开销

    // 10. 泛型常量表达式（Rust 1.51+）
    let arr: [i32; 3] = [1, 2, 3];
    let first = first_element(&arr);
    println!("数组第一个元素: {}", first);

    // 11. 泛型关联类型（GATs）
    let container = Container { value: 42 };
    let doubled = container.map(|x| x * 2);
    println!("容器值加倍: {}", doubled.value);

    // 12. 泛型中的常量泛型参数
    let buffer: Buffer<32> = Buffer::new();
    println!("缓冲区容量: {}", buffer.capacity());

    // 13. 泛型代码组织
    let numbers = vec![1, 2, 3, 4, 5];
    let stats = calculate_statistics(&numbers);
    println!("统计信息: {:?}", stats);
}

// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 多个泛型参数
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// 为泛型结构体实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只为特定类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 使用 trait bounds
pub trait Summary {
    fn summarize(&self) -> String;
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

// 使用 trait bound 的泛型函数
fn summarize<T: Summary>(item: &T) -> String {
    item.summarize()
}

// 多个 trait bounds
fn notify<T: Summary + std::fmt::Display>(item: &T) {
    println!("通知: {}", item.summarize());
}

// 使用 where 子句简化
fn compare_and_print<T>(t: &T, u: &T) -> String
where
    T: std::fmt::Display + PartialOrd,
{
    if t > u {
        format!("{} > {}", t, u)
    } else if t < u {
        format!("{} < {}", t, u)
    } else {
        format!("{} == {}", t, u)
    }
}

// 泛型与生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 泛型常量表达式
fn first_element<T, const N: usize>(arr: &[T; N]) -> &T {
    &arr[0]
}

// 泛型关联类型（GATs）示例
trait Map {
    type Output;
    fn map<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(i32) -> i32;
}

struct Container {
    value: i32,
}

impl Map for Container {
    type Output = Container;

    fn map<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(i32) -> i32,
    {
        Container {
            value: f(self.value),
        }
    }
}

// 常量泛型参数
struct Buffer<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> Buffer<SIZE> {
    fn new() -> Self {
        Buffer {
            data: [0; SIZE],
        }
    }

    fn capacity(&self) -> usize {
        SIZE
    }
}

// 泛型代码组织示例
#[derive(Debug)]
struct Statistics {
    min: f64,
    max: f64,
    mean: f64,
    median: f64,
}

fn calculate_statistics<T>(numbers: &[T]) -> Statistics
where
    T: Into<f64> + Copy + PartialOrd,
{
    if numbers.is_empty() {
        return Statistics {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: 0.0,
        };
    }

    let mut floats: Vec<f64> = numbers.iter().map(|&n| n.into()).collect();
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = floats[0];
    let max = floats[floats.len() - 1];
    let sum: f64 = floats.iter().sum();
    let mean = sum / floats.len() as f64;

    let median = if floats.len() % 2 == 0 {
        let mid = floats.len() / 2;
        (floats[mid - 1] + floats[mid]) / 2.0
    } else {
        floats[floats.len() / 2]
    };

    Statistics {
        min,
        max,
        mean,
        median,
    }
}

// 泛型的高级用法：类型状态模式
struct RequestBuilder<State> {
    url: String,
    method: String,
    state: std::marker::PhantomData<State>,
}

struct NoBody;
struct WithBody {
    body: String,
}

impl RequestBuilder<NoBody> {
    fn new(url: String) -> Self {
        RequestBuilder {
            url,
            method: "GET".to_string(),
            state: std::marker::PhantomData,
        }
    }

    fn with_body(self, body: String) -> RequestBuilder<WithBody> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            state: std::marker::PhantomData,
        }
    }
}

impl RequestBuilder<WithBody> {
    fn send(self) {
        println!("发送 {} 请求到 {}，包含正文", self.method, self.url);
    }
}

// 使用示例
fn use_request_builder() {
    let request = RequestBuilder::new("https://example.com".to_string())
        .with_body("Hello, World!".to_string());
    request.send();
}

// 泛型约束的另一种写法：使用 impl Trait
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 泛型中的默认类型参数
trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
struct Meters(f64);

impl Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

// 在 main 中测试
// fn main() {
//     use_request_builder();
//
//     let tweet = return_summarizable();
//     println!("返回的推文: {}", tweet.summarize());
//
//     let distance1 = Meters(5.0);
//     let distance2 = Meters(3.0);
//     let total = distance1.add(distance2);
//     println!("总距离: {:?} 米", total);
// }