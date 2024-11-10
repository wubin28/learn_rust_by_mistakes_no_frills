// 从标准库导入RefCell，这是一个提供内部可变性的cell类型
use std::cell::RefCell;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 0, y: 0 };

    // 将不可变实例包装在RefCell中，
    // 通过RefCell提供的内部可变性机制来实现后续对包装值的修改
    let point = RefCell::new(point);

    // 使用borrow()获取内部值的不可变引用,并打印初始坐标
    println!("初始点: ({}, {})", point.borrow().x, point.borrow().y);

    // 使用borrow_mut()获取内部值的可变引用,修改 x 坐标为 5
    point.borrow_mut().x = 5;

    println!(
        "尝试修改后的点: ({}, {})\n",
        point.borrow().x,
        point.borrow().y
    );
}
// 运行输出：
// 初始点: (0, 0)
// 尝试修改后的点: (5, 0)
