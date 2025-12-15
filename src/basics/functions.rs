//! 函数
//!
//! Rust 代码中的函数和变量名使用 snake_case 规范。
//! 函数使用 `fn` 关键字定义，可以有参数和返回值。

pub fn main() {
    println!("=== 函数示例 ===");

    // 1. 无参数无返回值的函数
    print_hello();

    // 2. 带参数的函数
    print_number(42);

    // 3. 带多个参数的函数
    print_sum(5, 3);

    // 4. 有返回值的函数
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // 5. 使用表达式作为返回值
    let squared = square(5);
    println!("5 的平方 = {}", squared);

    // 6. 返回多个值（使用元组）
    let (sum, diff) = add_and_subtract(10, 4);
    println!("10 + 4 = {}, 10 - 4 = {}", sum, diff);

    // 7. 函数指针
    let func_ptr: fn(i32, i32) -> i32 = add;
    println!("通过函数指针调用: {}", func_ptr(3, 7));

    // 8. 高阶函数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("加倍后的数组: {:?}", doubled);
}

// 无参数无返回值的函数
fn print_hello() {
    println!("Hello, Rust!");
}

// 带一个参数的函数
fn print_number(x: i32) {
    println!("数字是: {}", x);
}

// 带两个参数的函数
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// 有返回值的函数（显式 return）
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// 有返回值的函数（隐式返回表达式）
fn square(x: i32) -> i32 {
    x * x  // 没有分号，这是一个表达式，会作为返回值
}

// 返回多个值（使用元组）
fn add_and_subtract(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// 函数中的控制流
fn absolute_value(x: i32) -> i32 {
    if x >= 0 {
        x  // 返回 x
    } else {
        -x // 返回 -x
    }
}

// 递归函数
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 闭包（匿名函数）
fn closure_example() {
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("闭包: 5 + 1 = {}", add_one(5));

    // 捕获环境变量
    let y = 10;
    let add_y = |x| x + y;
    println!("捕获环境变量: 5 + {} = {}", y, add_y(5));
}