// 使用struct关键字声明了名为Point的结构体
struct Point {
    // 定义结构体字段x，类型为32位有符号整数
    x: i32,
    // 定义结构体字段y，类型为32位有符号整数
    y: i32,
}

fn main() {
    // 创建Point结构体的实例，将x和y都初始化为0,
    // 注意这里将结构体实例绑定到不可变变量point上
    let point = Point { x: 0, y: 0 };
    // 打印初始点坐标
    println!("初始点: ({}, {})", point.x, point.y);

    // 试图修改结构体实例字段x的值
    point.x = 5;

    println!("尝试修改后的点: ({}, {})\n", point.x, point.y);
}
// 编译错误：
// error[E0594]: cannot assign to `point.x`, as `point` is not declared as mutable
