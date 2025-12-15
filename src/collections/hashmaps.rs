//! 哈希映射（Hash Maps）
//!
//! HashMap<K, V> 存储键值对，使用哈希函数确定键的存储位置。
//! 键必须是可哈希的类型，值可以是任何类型。

use std::collections::HashMap;

pub fn main() {
    println!("=== 哈希映射基础 ===");

    // 1. 创建哈希映射
    let mut scores = HashMap::new();

    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("初始映射: {:?}", scores);

    // 2. 从向量创建哈希映射
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从向量创建: {:?}", scores);

    // 3. 访问值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("{} 队的分数: {}", team_name, s),
        None => println!("没有找到 {} 队", team_name),
    }

    // 4. 遍历哈希映射
    println!("\n遍历键值对:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("\n只遍历键:");
    for key in scores.keys() {
        println!("键: {}", key);
    }

    println!("\n只遍历值:");
    for value in scores.values() {
        println!("值: {}", value);
    }

    // 5. 更新哈希映射
    let mut scores = HashMap::new();

    // 覆盖已有的值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 覆盖 10
    println!("覆盖后: {:?}", scores);

    // 只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 不会覆盖，因为 Blue 已存在
    println!("使用 entry 后: {:?}", scores);

    // 6. 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);

    // 7. 哈希映射方法
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // 长度和容量
    println!("长度: {}, 是否为空: {}", map.len(), map.is_empty());

    // 检查键是否存在
    println!("包含键 'a': {}", map.contains_key("a"));
    println!("包含键 'd': {}", map.contains_key("d"));

    // 移除键值对
    map.remove("b");
    println!("移除 'b' 后: {:?}", map);

    // 清空哈希映射
    map.clear();
    println!("清空后长度: {}", map.len());

    // 8. 哈希映射和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 不再有效

    // 9. 使用引用避免所有权转移
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("使用引用: {:?}", map);
    println!("原变量仍然有效: {}, {}", field_name, field_value);

    // 10. 哈希映射的哈希函数
    // 默认使用加密安全的哈希函数，速度较慢但安全
    // 可以使用其他哈希器（hasher）来改变性能特征

    // 11. 复杂值的哈希映射
    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut people = HashMap::new();
    people.insert(
        Person { name: String::from("Alice"), age: 30 },
        "Engineer"
    );
    people.insert(
        Person { name: String::from("Bob"), age: 25 },
        "Designer"
    );
    println!("复杂键的映射: {:?}", people);

    // 12. 哈希映射的默认值
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();

    // 如果键不存在，插入空向量
    map.entry(String::from("scores")).or_insert_with(Vec::new).push(100);
    map.entry(String::from("scores")).or_insert_with(Vec::new).push(200);
    println!("默认值示例: {:?}", map);

    // 13. 合并两个哈希映射
    let mut map1 = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);

    let mut map2 = HashMap::new();
    map2.insert("b", 3);
    map2.insert("c", 4);

    // 扩展 map1
    map1.extend(map2);
    println!("合并后: {:?}", map1); // b 被覆盖为 3

    // 14. 哈希映射查找和修改
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    // 查找并修改
    if let Some(value) = map.get_mut("key1") {
        *value = "new_value1";
    }
    println!("修改后: {:?}", map);

    // 15. 哈希映射的容量管理
    let mut map: HashMap<String, i32> = HashMap::with_capacity(10);
    println!("初始容量: {}", map.capacity());

    for i in 0..15 {
        map.insert(format!("key{}", i), i);
    }
    println!("添加15个元素后容量: {}", map.capacity());

    map.shrink_to_fit();
    println!("收缩后容量: {}", map.capacity());

    // 16. 哈希映射的迭代器方法
    let map: HashMap<_, _> = [("a", 1), ("b", 2), ("c", 3)].iter().cloned().collect();

    // map: 转换键值对
    let doubled: HashMap<_, _> = map.iter()
        .map(|(k, v)| (k.clone(), v * 2))
        .collect();
    println!("加倍值: {:?}", doubled);

    // filter: 过滤键值对
    let filtered: HashMap<_, _> = map.into_iter()
        .filter(|(k, _)| k != &"b")
        .collect();
    println!("过滤后: {:?}", filtered);

    // 17. 哈希映射序列化和反序列化
    // 通常使用 serde 库，这里展示基本概念
    let map: HashMap<String, i32> = [
        ("apple".to_string(), 3),
        ("banana".to_string(), 2),
    ].iter().cloned().collect();
    println!("可序列化的映射: {:?}", map);

    // 18. 性能考虑
    // - 哈希映射在查找、插入、删除方面平均 O(1) 时间复杂度
    // - 但最坏情况是 O(n)
    // - 对于小数据集，BTreeMap 可能更快
    // - 键的选择影响哈希碰撞和性能
}

// 自定义哈希函数示例
fn custom_hash_example() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[derive(Debug)]
    struct CustomKey {
        id: u32,
        name: String,
    }

    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            // 可以选择不哈希 name 来加速
            // 或者只哈希 name 的一部分
        }
    }

    impl PartialEq for CustomKey {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
            // 根据哈希实现决定相等性逻辑
        }
    }

    impl Eq for CustomKey {}

    let mut map = HashMap::new();
    map.insert(CustomKey { id: 1, name: "Alice".to_string() }, "value1");
    map.insert(CustomKey { id: 2, name: "Bob".to_string() }, "value2");

    // 查找时只比较 id
    let key = CustomKey { id: 1, name: "DifferentName".to_string() };
    println!("查找结果: {:?}", map.get(&key));
}