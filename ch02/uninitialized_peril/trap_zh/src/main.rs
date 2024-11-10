fn main() {
    // 声明了不可变变量x，但暂未初始化
    let x: i32;
    // true是一个布尔字面值，表示条件永远成立
    if true {
        // 将不可变变量x初始化为5，
        // 因为条件为true，所以这行代码一定会执行
        x = 5;
    }
    println!("x的值是: {}", x);
}
// 编译错误：
// error[E0381]: used binding `x` is possibly-uninitialized
