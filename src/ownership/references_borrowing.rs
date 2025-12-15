//! 引用和借用
//!
//! 引用（reference）允许你使用值但不获取其所有权。
//! 借用（borrowing）是创建引用的行为。
//! 规则：
//! 1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
//! 2. 引用必须总是有效的。

pub fn main() {
    println!("=== 引用和借用 ===");

    // 1. 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用，不获取所有权
    println!("'{}' 的长度是 {}", s1, len);

    // 2. 可变引用
    let mut s = String::from("hello");
    change(&mut s); // 传递可变引用
    println!("修改后: {}", s);

    // 3. 引用规则示例
    let mut s = String::from("hello");

    let r1 = &s; // 没问题：不可变引用
    let r2 = &s; // 没问题：多个不可变引用
    println!("{} 和 {}", r1, r2);
    // r1 和 r2 的作用域在这里结束

    let r3 = &mut s; // 没问题：没有不可变引用同时存在
    println!("{}", r3);

    // 4. 数据竞争预防
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // 错误：不能同时有两个可变引用
    // println!("{}, {}", r1, r2);

    // 5. 作用域的重要性
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("在内部作用域中: {}", r1);
    } // r1 离开作用域，所以可以创建新的引用

    let r2 = &mut s;
    println!("在外部作用域中: {}", r2);

    // 6. 不可变引用和可变引用不能同时存在
    let mut s = String::from("hello");

    let r1 = &s; // 没问题：不可变引用
    let r2 = &s; // 没问题：不可变引用
    // let r3 = &mut s; // 错误：不能在有不可变引用的同时创建可变引用
    println!("{} 和 {}", r1, r2);
    // r1 和 r2 的作用域在这里结束

    let r3 = &mut s; // 没问题：不可变引用已经不再使用
    println!("{}", r3);

    // 7. 悬垂引用（Dangling References）
    // let reference_to_nothing = dangle(); // 错误：返回了悬垂引用

    let no_dangle = no_dangle(); // 正确：返回 String，转移所有权
    println!("没有悬垂: {}", no_dangle);

    // 8. 切片引用
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("切片: '{}' 和 '{}'", hello, world);

    // 9. 结构体引用
    let mut point = Point { x: 0, y: 0 };
    let r = &mut point;
    r.x = 5;
    r.y = 10;
    println!("点: ({}, {})", point.x, point.y);

    // 10. 引用作为函数参数和返回值
    let s = String::from("hello");
    let first_word = first_word(&s);
    println!("第一个单词: {}", first_word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它是引用，不会丢弃任何东西

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 错误示例：悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 错误：返回局部变量的引用
// } // s 离开作用域并被丢弃，引用指向无效内存

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 正确：返回 String，转移所有权
}

struct Point {
    x: i32,
    y: i32,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 示例：多个返回值使用引用
fn split_at_space(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], &s[i + 1..]),
        None => (s, ""),
    }
}