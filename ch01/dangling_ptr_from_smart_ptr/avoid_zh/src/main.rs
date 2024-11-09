fn main() {
    let raw_ptr;
    let smart_ptr;

    {
        smart_ptr = Box::new(55);
        raw_ptr = &*smart_ptr as *const i32;

        println!("智能指针管理的值: {}", smart_ptr);
        unsafe {
            println!("裸指针指向的值: {}", *raw_ptr);
        }
    }

    unsafe {
        println!(
            "尝试访问悬垂裸指针指向的值: {}",
            *raw_ptr
        );
    }
}
// 运行输出:
// 智能指针管理的值: 55
// 裸指针指向的值: 55
// 尝试访问悬垂裸指针指向的值: 55