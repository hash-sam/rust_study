//! 错误处理
//!
//! Rust 将错误分为两大类：可恢复错误（Result<T, E>）和不可恢复错误（panic!）。
//! Rust 没有异常，而是使用 Result 类型和 panic 宏来处理错误。

use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;

pub fn main() {
    println!("=== 错误处理示例 ===");

    // 1. panic! 宏 - 不可恢复错误
    // panic!("严重错误!"); // 这会终止程序

    // 2. Result 枚举 - 可恢复错误
    let file_result = File::open("hello.txt");

    match file_result {
        Ok(file) => {
            println!("文件打开成功: {:?}", file);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("文件未找到，尝试创建...");
                match File::create("hello.txt") {
                    Ok(fc) => println!("文件创建成功: {:?}", fc),
                    Err(e) => println!("创建文件时出错: {:?}", e),
                }
            }
            other_error => {
                println!("打开文件时出错: {:?}", other_error);
            }
        },
    }

    // 3. unwrap 方法 - 成功时返回值，错误时 panic
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error);
        }
    });
    println!("使用 unwrap_or_else: {:?}", file);

    // 4. expect 方法 - 类似 unwrap，但可以指定错误信息
    let file = File::open("hello.txt").expect("打开 hello.txt 文件失败");
    println!("使用 expect: {:?}", file);

    // 5. 传播错误
    let username_result = read_username_from_file();
    match username_result {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取用户名失败: {}", e),
    }

    // 6. ? 运算符 - 错误传播的语法糖
    let username = read_username_from_file_with_question().unwrap_or_else(|e| {
        println!("读取失败: {}", e);
        String::from("默认用户")
    });
    println!("使用 ? 运算符读取的用户名: {}", username);

    // 7. 自定义错误类型
    let result = parse_positive_number("42");
    match result {
        Ok(num) => println!("解析的正数: {}", num),
        Err(e) => println!("解析错误: {}", e),
    }

    let result = parse_positive_number("-5");
    match result {
        Ok(num) => println!("解析的正数: {}", num),
        Err(e) => println!("解析错误: {}", e),
    }

    // 8. 错误类型转换
    let result = parse_number_then_double("42");
    match result {
        Ok(num) => println!("解析并加倍: {}", num),
        Err(e) => println!("错误: {}", e),
    }

    // 9. 错误链
    let result = read_config_file();
    match result {
        Ok(config) => println!("配置: {}", config),
        Err(e) => println!("读取配置失败: {}", e),
    }

    // 10. 组合错误处理
    let numbers = vec!["1", "2", "three", "4"];
    let parsed: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();
    match parsed {
        Ok(nums) => println!("解析成功: {:?}", nums),
        Err(e) => println!("解析失败: {}", e),
    }

    // 11. 错误处理最佳实践
    // - 使用 Result 处理可恢复错误
    // - 使用 panic! 处理不可恢复错误（bug）
    // - 提供有意义的错误信息
    // - 使用 ? 运算符简化错误传播
    // - 定义自己的错误类型以提高类型安全性
}

// 传播错误的函数
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = match File::open("username.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 使用 ? 运算符简化错误传播
fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 更简洁的写法
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 自定义错误类型
#[derive(Debug)]
enum ParseError {
    InvalidNumber(ParseIntError),
    NegativeNumber,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidNumber(e) => write!(f, "无效的数字: {}", e),
            ParseError::NegativeNumber => write!(f, "数字不能为负数"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::InvalidNumber(e) => Some(e),
            ParseError::NegativeNumber => None,
        }
    }
}

fn parse_positive_number(s: &str) -> Result<i32, ParseError> {
    let num = s.parse::<i32>().map_err(ParseError::InvalidNumber)?;

    if num < 0 {
        Err(ParseError::NegativeNumber)
    } else {
        Ok(num)
    }
}

// 错误类型转换
fn parse_number_then_double(s: &str) -> Result<i32, String> {
    let num = s.parse::<i32>().map_err(|e| format!("解析失败: {}", e))?;
    Ok(num * 2)
}

// 错误链示例
fn read_config_file() -> Result<String, Box<dyn std::error::Error>> {
    let path = "config.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("无法读取文件 {}: {}", path, e))?;

    let config = parse_config(&content)
        .map_err(|e| format!("解析配置失败: {}", e))?;

    Ok(config)
}

fn parse_config(content: &str) -> Result<String, &str> {
    if content.is_empty() {
        Err("配置文件为空")
    } else {
        Ok(content.to_string())
    }
}

// 示例：多种错误类型处理
fn process_data(data: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // 可能产生 io::Error
    let file_content = std::fs::read_to_string(data)?;

    // 可能产生 ParseIntError
    let number: i32 = file_content.trim().parse()?;

    // 自定义验证
    if number > 100 {
        return Err("数字太大".into());
    }

    Ok(number * 2)
}

// 示例：Option 和 Result 的转换
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn parse_and_find_even(strings: &[&str]) -> Result<Option<i32>, ParseIntError> {
    let numbers: Result<Vec<i32>, ParseIntError> =
        strings.iter().map(|s| s.parse()).collect();

    numbers.map(|nums| find_first_even(&nums))
}

// 示例：错误处理辅助函数
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

fn safe_divide(a: f64, b: f64) -> f64 {
    divide(a, b).unwrap_or_else(|e| {
        println!("警告: {}", e);
        0.0 // 返回默认值
    })
}

// 示例：panic 钩子
fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        println!("自定义 panic 处理:");
        if let Some(location) = panic_info.location() {
            println!("在 {}:{}:{} 发生 panic",
                location.file(),
                location.line(),
                location.column()
            );
        }

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic 信息: {}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            println!("panic 信息: {}", s);
        }
    }));
}

// 在 main 中测试
// fn main() {
//     setup_panic_hook();
//
//     println!("10 / 2 = {}", safe_divide(10.0, 2.0));
//     println!("10 / 0 = {}", safe_divide(10.0, 0.0));
//
//     let result = parse_and_find_even(&["1", "2", "3", "4"]);
//     println!("查找偶数结果: {:?}", result);
//
//     let result = process_data("data.txt");
//     match result {
//         Ok(value) => println!("处理结果: {}", value),
//         Err(e) => println!("处理失败: {}", e),
//     }
// }