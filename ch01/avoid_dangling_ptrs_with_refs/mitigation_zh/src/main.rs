fn main() {
    let reference;
    let smart_ptr;

    {
        smart_ptr = Box::new(55);
        reference = &*smart_ptr;

        println!("智能指针管理的值: {}", smart_ptr);
        println!("引用指向的值: {}", reference);
    }

    println!("引用指向的值: {}", reference);
}
// 运行输出:
// 智能指针管理的值: 55
// 引用指向的值: 55
// 引用指向的值: 55
