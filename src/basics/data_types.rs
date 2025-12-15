//! 数据类型
//!
//! Rust 是静态类型语言，在编译时必须知道所有变量的类型。
//! 数据类型分为两类：标量类型（scalar）和复合类型（compound）。

pub fn main() {
    // ========== 标量类型（Scalar Types） ==========

    // 1. 整数类型
    // 有符号整数：i8, i16, i32, i64, i128, isize
    // 无符号整数：u8, u16, u32, u64, u128, usize
    let integer: i32 = 42;
    let unsigned: u32 = 42;
    println!("有符号整数: {}, 无符号整数: {}", integer, unsigned);

    // 2. 浮点数类型
    // f32: 单精度浮点数，f64: 双精度浮点数（默认）
    let float32: f32 = 3.14;
    let float64: f64 = 3.141592653589793;
    println!("单精度浮点数: {}, 双精度浮点数: {}", float32, float64);

    // 3. 布尔类型
    let true_value: bool = true;
    let false_value: bool = false;
    println!("布尔值: {}, {}", true_value, false_value);

    // 4. 字符类型
    // char 类型表示单个 Unicode 标量值，占用 4 个字节
    let letter: char = 'A';
    let emoji: char = '😀';
    let chinese: char = '中';
    println!("字符: {}, {}, {}", letter, emoji, chinese);

    // ========== 复合类型（Compound Types） ==========

    // 5. 元组（Tuple）
    // 元组可以将多个不同类型的值组合成一个复合类型
    let tuple: (i32, f64, char) = (500, 6.4, 'Z');
    println!("元组: ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    // 元组解构
    let (x, y, z) = tuple;
    println!("解构元组: x={}, y={}, z={}", x, y, z);

    // 6. 数组（Array）
    // 数组中的元素必须是相同类型，长度固定
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("数组: {:?}", array);
    println!("第一个元素: {}", array[0]);
    println!("最后一个元素: {}", array[array.len() - 1]);

    // 创建相同值的数组
    let same_values = [3; 5]; // 等价于 [3, 3, 3, 3, 3]
    println!("相同值数组: {:?}", same_values);

    // 7. 切片（Slice）
    // 切片是对数组一部分的引用
    let slice = &array[1..4]; // 索引 1 到 3（不包括 4）
    println!("切片: {:?}", slice);

    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("字符串切片: '{}' 和 '{}'", hello, world);

    // ========== 类型转换 ==========

    // 8. 显式类型转换
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    println!("类型转换: {} as u8 = {}", decimal, integer);

    // 9. 类型推断
    let inferred = 42; // 编译器推断为 i32
    let inferred_float = 3.14; // 编译器推断为 f64
    println!("推断类型: {}, {}", inferred, inferred_float);
}