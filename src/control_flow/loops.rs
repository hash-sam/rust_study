//! 循环
//!
//! Rust 有三种循环：loop、while 和 for。
//! loop 是无限循环，while 是条件循环，for 是迭代循环。

pub fn main() {
    println!("=== loop 循环 ===");

    // 1. loop 无限循环
    let mut count = 0;
    loop {
        count += 1;
        println!("loop 计数: {}", count);

        if count == 3 {
            break; // 跳出循环
        }
    }

    // 2. loop 返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 从循环中返回值
        }
    };
    println!("loop 返回值: {}", result);

    println!("\n=== while 循环 ===");

    // 3. while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("发射!");

    // 4. while 遍历数组
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }

    println!("\n=== for 循环 ===");

    // 5. for 遍历数组（推荐方式）
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("值: {}", element);
    }

    // 6. for 遍历范围
    for number in 1..4 {
        println!("{}!", number);
    }

    // 7. for 遍历范围（包含结束值）
    for number in 1..=4 {
        println!("包含结束值: {}", number);
    }

    // 8. for 反向遍历
    for number in (1..4).rev() {
        println!("反向: {}!", number);
    }

    // 9. for 遍历字符串字符
    let s = "hello";
    for c in s.chars() {
        println!("字符: {}", c);
    }

    // 10. for 遍历索引和值
    let a = [10, 20, 30];
    for (index, value) in a.iter().enumerate() {
        println!("a[{}] = {}", index, value);
    }

    println!("\n=== 循环控制 ===");

    // 11. continue 跳过当前迭代
    for number in 1..10 {
        if number % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("奇数: {}", number);
    }

    // 12. 嵌套循环和标签
    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            if x == 2 && y == 2 {
                println!("在 ({}, {}) 处跳出外层循环", x, y);
                break 'outer;
            }
            if x == 1 && y == 1 {
                println!("在 ({}, {}) 处继续外层循环", x, y);
                continue 'outer;
            }
            println!("({}, {})", x, y);
        }
    }

    println!("\n=== 迭代器方法 ===");

    // 13. 使用迭代器方法
    let numbers = vec![1, 2, 3, 4, 5];

    // map: 转换每个元素
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("加倍: {:?}", doubled);

    // filter: 过滤元素
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // fold: 累积计算
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("总和: {}", sum);

    // 14. for 循环中的模式匹配
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in pairs {
        println!("{}: {}", num, word);
    }
}