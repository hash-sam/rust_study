//! 字符串（Strings）
//!
//! Rust 有两种主要的字符串类型：String 和 &str。
//! String 是可增长、可修改、拥有所有权的 UTF-8 编码字符串。
//! &str 是字符串切片，是对 UTF-8 编码字符串的引用。

pub fn main() {
    println!("=== 字符串基础 ===");

    // 1. 创建字符串
    let s1 = String::new(); // 空字符串
    println!("空字符串: '{}'", s1);

    let s2 = "初始内容".to_string(); // 从字面值创建
    println!("to_string: '{}'", s2);

    let s3 = String::from("hello"); // 使用 from 函数
    println!("String::from: '{}'", s3);

    // 2. 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar"); // 追加字符串切片
    println!("push_str 后: '{}'", s);

    s.push('!'); // 追加单个字符
    println!("push 后: '{}'", s);

    // 3. 字符串连接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意：s1 被移动，不能再使用
    println!("连接后: '{}'", s3);

    // 使用 format! 宏（不会获取所有权）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format! 宏: '{}'", s);
    println!("原字符串仍然可用: {}, {}, {}", s1, s2, s3);

    // 4. 字符串索引
    let s = String::from("hello");
    // let h = s[0]; // 错误：不能直接索引字符串

    // 5. 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 每个 Unicode 标量值占 2 字节
    println!("字符串切片: '{}'", s);

    // 6. 遍历字符串
    let s = "नमस्ते";

    // 遍历字符
    println!("遍历字符:");
    for c in s.chars() {
        println!("{}", c);
    }

    // 遍历字节
    println!("\n遍历字节:");
    for b in s.bytes() {
        println!("{}", b);
    }

    // 7. 字符串方法
    let s = String::from("Hello, world!");

    // 长度（字节数）
    println!("长度: {}", s.len());

    // 是否为空
    println!("是否为空: {}", s.is_empty());

    // 包含子串
    println!("包含 'world': {}", s.contains("world"));

    // 查找子串
    println!("'world' 的索引: {:?}", s.find("world"));

    // 替换
    let replaced = s.replace("world", "Rust");
    println!("替换后: '{}'", replaced);

    // 大小写转换
    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
    println!("大写: '{}'", upper);
    println!("小写: '{}'", lower);

    // 8. 字符串分割
    let s = "apple,banana,cherry";
    let fruits: Vec<&str> = s.split(',').collect();
    println!("分割水果: {:?}", fruits);

    // 多分隔符分割
    let s = "apple and banana or cherry";
    let fruits: Vec<&str> = s.split(|c| c == ' ' || c == 'a').collect();
    println!("复杂分割: {:?}", fruits);

    // 9. 字符串修剪
    let s = "   hello world   \n";
    println!("原始: '{}'", s);
    println!("修剪两端空白: '{}'", s.trim());
    println!("修剪开头空白: '{}'", s.trim_start());
    println!("修剪结尾空白: '{}'", s.trim_end());

    // 10. 字符串解析
    let s = "42";
    let number: i32 = s.parse().unwrap();
    println!("解析数字: {}", number);

    let s = "3.14";
    let pi: f64 = s.parse().unwrap();
    println!("解析浮点数: {}", pi);

    // 11. 字符串和字符操作
    let mut s = String::new();

    // 插入字符
    s.insert(0, 'H');
    s.insert(1, 'i');
    println!("插入后: '{}'", s);

    // 插入字符串
    s.insert_str(1, "ello");
    println!("插入字符串后: '{}'", s);

    // 移除字符
    s.remove(0);
    println!("移除后: '{}'", s);

    // 12. 字符串比较
    let s1 = "hello";
    let s2 = "HELLO";
    let s3 = "hello";

    println!("s1 == s2: {}", s1 == s2);
    println!("s1 == s3: {}", s1 == s3);
    println!("s1.eq_ignore_ascii_case(s2): {}", s1.eq_ignore_ascii_case(&s2));

    // 13. 字符串排序
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort();
    println!("排序单词: {:?}", words);

    // 14. 字符串和字节数组转换
    let s = "hello";
    let bytes = s.as_bytes();
    println!("字节数组: {:?}", bytes);

    let from_bytes = String::from_utf8(bytes.to_vec()).unwrap();
    println!("从字节数组恢复: '{}'", from_bytes);

    // 15. 原始字符串
    let raw_string = r#"这是一个"原始"字符串，可以包含引号"#;
    println!("原始字符串: {}", raw_string);

    let multi_line_raw = r#"
        多行
        原始
        字符串
    "#;
    println!("多行原始字符串: {}", multi_line_raw);

    // 16. 字符串格式化
    let name = "Alice";
    let age = 30;
    let formatted = format!("{} 今年 {} 岁", name, age);
    println!("格式化: {}", formatted);

    // 带格式的格式化
    let pi = 3.141592653589793;
    println!("PI: {:.2}", pi); // 保留两位小数
    println!("PI: {:10.2}", pi); // 宽度10，保留两位小数
    println!("PI: {:<10.2}", pi); // 左对齐
    println!("PI: {:^10.2}", pi); // 居中对齐
    println!("PI: {:>10.2}", pi); // 右对齐

    // 17. 字符串和所有权
    let s1 = String::from("hello");
    let s2 = s1; // 所有权转移
    // println!("{}", s1); // 错误：s1 不再有效

    let s3 = s2.clone(); // 深度拷贝
    println!("克隆: {}", s3);

    // 18. 字符串切片作为函数参数
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("第一个单词: '{}'", word);

    // 字符串字面值就是切片
    let word = first_word("hello world");
    println!("字符串字面值的第一个单词: '{}'", word);
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

// 字符串处理函数示例
fn string_operations() {
    // 检查字符串前缀和后缀
    let s = "hello world";
    println!("以 'hello' 开头: {}", s.starts_with("hello"));
    println!("以 'world' 结尾: {}", s.ends_with("world"));

    // 重复字符串
    let repeated = "ha".repeat(3);
    println!("重复: {}", repeated);

    // 字符串填充
    let s = "42";
    println!("左填充: '{:0>5}'", s); // 00042
    println!("右填充: '{:*<5}'", s); // 42***

    // 字符串转义
    let escaped = "Line 1\nLine 2\tTab\rCarriage return";
    println!("转义字符: {}", escaped);
}