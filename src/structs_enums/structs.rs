//! 结构体（Structs）
//!
//! 结构体是一种自定义数据类型，允许你将多个相关的值打包在一起，
//! 并给每个值命名以提供清晰的语义。

pub fn main() {
    println!("=== 结构体基础 ===");

    // 1. 定义和实例化结构体
    let user1 = User {
        username: String::from("john_doe"),
        email: String::from("john@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("用户: {}", user1.username);

    // 2. 修改结构体字段（需要 mut）
    let mut user2 = User {
        username: String::from("jane_doe"),
        email: String::from("jane@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user2.sign_in_count = 2;
    println!("修改后的登录次数: {}", user2.sign_in_count);

    // 3. 结构体更新语法
    let user3 = User {
        username: String::from("bob_smith"),
        email: String::from("bob@example.com"),
        ..user1 // 使用 user1 的剩余字段
    };
    println!("新用户: {}, 活跃: {}", user3.username, user3.active);

    // 4. 元组结构体（Tuple Structs）
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("颜色: ({}, {}, {})", black.0, black.1, black.2);

    // 5. 类单元结构体（Unit-like Structs）
    let subject = AlwaysEqual;
    println!("类单元结构体实例化");

    // 6. 结构体方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("矩形面积: {}", rect1.area());
    println!("能容纳 rect2 吗? {}", rect1.can_hold(&Rectangle { width: 10, height: 40 }));

    // 7. 关联函数（类似静态方法）
    let square = Rectangle::square(10);
    println!("正方形: {}x{}", square.width, square.height);

    // 8. 结构体所有权
    let user4 = build_user(
        String::from("alice"),
        String::from("alice@example.com"),
    );
    println!("构建的用户: {}", user4.username);

    // 9. 打印结构体（使用 Debug trait）
    #[derive(Debug)]
    struct DebugPoint {
        x: i32,
        y: i32,
    }

    let debug_point = DebugPoint { x: 5, y: 10 };
    println!("调试输出: {:?}", debug_point);
    println!("美化输出: {:#?}", debug_point);

    // 10. 结构体模式匹配
    let point = Point3D { x: 0, y: 0, z: 0 };
    match point {
        Point3D { x: 0, y: 0, z: 0 } => println!("原点"),
        Point3D { x, y, z } => println!("点: ({}, {}, {})", x, y, z),
    }
}

// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;

// 带有方法的结构体
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法：第一个参数总是 self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 另一个方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数：没有 self 参数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 多个 impl 块
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// 返回结构体的函数
fn build_user(username: String, email: String) -> User {
    User {
        username, // 字段初始化简写语法
        email,    // 参数名和字段名相同时可以省略
        sign_in_count: 1,
        active: true,
    }
}

// 用于模式匹配的结构体
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// 结构体中的生命周期注解（后续会详细讲解）
struct ImportantExcerpt<'a> {
    part: &'a str,
}