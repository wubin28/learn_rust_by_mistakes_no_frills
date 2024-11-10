fn main() {
    let x: i32;
    if true {
        x = 5;
    } else {
        unreachable!();
    }
    println!("x的值是: {}", x);
}
// 运行输出：
// x的值是: 5
