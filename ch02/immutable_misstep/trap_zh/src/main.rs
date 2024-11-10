fn main() {
    // 声明了不可变变量sum并初始化为0，
    // 也就是将0绑定到不可变变量sum上
    let sum = 0;
    // 使用for循环遍历1到3的范围
    // ..符号是Rust的范围语法
    // =符号表示这是一个包含上界的范围
    for i in 1..=3 {
        // 相当于sum = sum + i;
        sum += i;
    }
    println!("求和结果: {}\n", sum);
}
// 编译错误:
// error[E0384]: cannot assign twice to immutable variable `sum`
