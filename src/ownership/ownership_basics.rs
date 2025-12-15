//! 所有权基础
//!
//! Rust 的核心特性是所有权系统，它保证了内存安全而无需垃圾回收。
//! 所有权规则：
//! 1. Rust 中的每一个值都有一个被称为其所有者的变量。
//! 2. 值在任一时刻有且只有一个所有者。
//! 3. 当所有者离开作用域，这个值将被丢弃。

pub fn main() {
    println!("=== 所有权基础 ===");

    // 1. 变量作用域
    {
        let s = "hello"; // s 从这里开始有效
        println!("s = {}", s);
    } // s 离开作用域，不再有效

    // 2. String 类型（堆上分配）
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 3. 移动（Move）
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2，s1 不再有效
    // println!("{}", s1); // 错误：s1 不再有效
    println!("s2 = {}", s2);

    // 4. 克隆（Clone） - 深度拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 创建数据的完整拷贝
    println!("s1 = {}, s2 = {}", s1, s2);

    // 5. 栈上数据的拷贝（Copy trait）
    let x = 5;
    let y = x; // x 是整数，实现了 Copy trait，所以是拷贝而不是移动
    println!("x = {}, y = {}", x, y);

    // 6. 函数与所有权
    let s = String::from("hello");
    takes_ownership(s); // s 的所有权移动到函数中
    // println!("{}", s); // 错误：s 不再有效

    let x = 5;
    makes_copy(x); // x 是 Copy 类型，所以是拷贝
    println!("x 仍然有效: {}", x); // x 仍然有效

    // 7. 返回值与所有权
    let s1 = gives_ownership(); // 函数返回值所有权转移给 s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 所有权转移，然后返回
    println!("s1 = {}, s3 = {}", s1, s3);
    // println!("s2 = {}", s2); // 错误：s2 不再有效
}

fn takes_ownership(some_string: String) {
    println!("获取所有权: {}", some_string);
} // some_string 离开作用域，drop 被调用，内存被释放

fn makes_copy(some_integer: i32) {
    println!("拷贝整数: {}", some_integer);
} // some_integer 离开作用域，没什么特别的事情发生

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回 some_string，所有权转移给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string，所有权转移给调用者
}

// 示例：计算字符串长度（不获取所有权）
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回元组，将所有权返回给调用者
}

// 示例：使用引用避免所有权转移
fn calculate_length_with_ref(s: &String) -> usize {
    s.len()
} // s 是引用，不获取所有权