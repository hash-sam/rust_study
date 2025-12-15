//! 向量（Vectors）
//!
//! Vec<T> 是 Rust 标准库中的可增长数组类型。
//! 向量在堆上分配内存，可以动态调整大小。

pub fn main() {
    println!("=== 向量基础 ===");

    // 1. 创建向量
    let v1: Vec<i32> = Vec::new(); // 空向量，需要类型注解
    println!("空向量: {:?}", v1);

    let v2 = vec![1, 2, 3]; // 使用宏创建并初始化
    println!("初始化向量: {:?}", v2);

    // 2. 更新向量
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("更新后: {:?}", v3);

    // 3. 读取向量元素
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引（可能 panic）
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    // 使用 get 方法（返回 Option）
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("没有第三个元素"),
    }

    // 4. 遍历向量
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 5. 遍历并修改
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 解引用并修改
    }
    println!("修改后: {:?}", v);

    // 6. 使用枚举存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 7. 向量方法
    let mut v = vec![1, 2, 3];

    // 长度和容量
    println!("长度: {}, 容量: {}", v.len(), v.capacity());

    // 检查是否为空
    println!("是否为空: {}", v.is_empty());

    // 插入元素
    v.insert(1, 10); // 在索引 1 处插入 10
    println!("插入后: {:?}", v);

    // 移除元素
    v.remove(1); // 移除索引 1 处的元素
    println!("移除后: {:?}", v);

    // 弹出最后一个元素
    let last = v.pop();
    println!("弹出: {:?}, 剩余: {:?}", last, v);

    // 8. 向量切片
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..4]; // 索引 1 到 3
    println!("切片: {:?}", slice);

    // 9. 连接向量
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = [v1, v2].concat();
    println!("连接后: {:?}", v3);

    // 或者使用 extend
    let mut v4 = vec![1, 2];
    v4.extend([3, 4].iter());
    println!("扩展后: {:?}", v4);

    // 10. 向量排序
    let mut v = vec![5, 2, 8, 1, 9];
    v.sort();
    println!("排序后: {:?}", v);

    v.sort_by(|a, b| b.cmp(a)); // 降序排序
    println!("降序排序: {:?}", v);

    // 11. 向量去重
    let mut v = vec![1, 2, 2, 3, 3, 3, 4];
    v.dedup();
    println!("去重后: {:?}", v);

    // 12. 向量查找
    let v = vec![10, 20, 30, 40, 50];
    let index = v.iter().position(|&x| x == 30);
    println!("30 的索引: {:?}", index);

    // 13. 向量过滤
    let v = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // 14. 向量映射
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("加倍: {:?}", doubled);

    // 15. 向量折叠
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("总和: {}", sum);

    let product: i32 = v.iter().product();
    println!("乘积: {}", product);

    // 16. 二维向量
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    println!("矩阵:");
    for row in &matrix {
        println!("{:?}", row);
    }

    // 17. 向量容量管理
    let mut v = Vec::with_capacity(10);
    println!("初始容量: {}", v.capacity());

    for i in 0..15 {
        v.push(i);
    }
    println!("添加15个元素后容量: {}", v.capacity());

    v.shrink_to_fit();
    println!("收缩后容量: {}", v.capacity());

    // 18. 向量和迭代器
    let v = vec!["a", "b", "c"];

    // 转换为迭代器
    let mut iter = v.iter();
    println!("第一个: {:?}", iter.next());
    println!("第二个: {:?}", iter.next());

    // 收集为向量
    let collected: Vec<&str> = v.iter().cloned().collect();
    println!("收集: {:?}", collected);
}