struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };
    println!("初始点: ({}, {})", point.x, point.y);

    point.x = 5;

    println!("尝试修改后的点: ({}, {})\n", point.x, point.y);
}
// 运行输出：
// 初始点: (0, 0)
// 尝试修改后的点: (5, 0)
