fn main() {
    // 声明一个变量，但暂不初始化
    let raw_ptr;

    { // 开始一个新作用域
        let smart_ptr = Box::new(55);
        // 从智能指针获取裸指针
        raw_ptr = &*smart_ptr as *const i32;

        println!("智能指针管理的值: {}", smart_ptr);
        unsafe {
            println!("裸指针指向的值: {}", *raw_ptr);
        }
    } // smart_ptr在此作用域结束后被销毁，其指向的内存被释放

    // 尝试使用悬垂指针raw_ptr
    unsafe {
        // 编译通过，运行时产生未定义行为
        println!(
            "尝试访问悬垂裸指针指向的值（每次运行都不一样）: {}",
            *raw_ptr
        );
    }
}
// 运行输出:
// 智能指针管理的值: 55
// 裸指针指向的值: 55
// 尝试访问悬垂裸指针指向的值（每次运行都不一样）: -693338112
