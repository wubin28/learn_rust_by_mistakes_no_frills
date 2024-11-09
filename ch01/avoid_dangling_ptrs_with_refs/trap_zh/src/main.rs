fn main() {
    // 声明名为reference的变量，但暂不初始化
    let reference;

    {
        // 开始一个新作用域
        // 绑定类型为Box<i32>的智能指针smart_ptr，存储整数值55，Box在堆上分配内存
        let smart_ptr = Box::new(55);
        // 变量reference的类型在这里进行初始化时才会被推断出来
        // 下面解释表达式&*smart_ptr：
        // 首先*解引用操作符将Box智能指针解引用到其在堆上管理的i32值
        // 然后&引用创建操作符创建了这个堆上值的引用
        // 最后将这个引用初始化给变量reference
        reference = &*smart_ptr;

        println!("智能指针管理的值: {}", smart_ptr);

        println!("引用指向的值: {}", reference);
    } // smart_ptr在此作用域结束后被销毁

    // 尝试使用悬垂引用reference会导致编译错误
    println!("引用指向的值: {}", reference);
}
// 'cargo build' 编译输出:
// error[E0597]: `*smart_ptr` does not live long enough
