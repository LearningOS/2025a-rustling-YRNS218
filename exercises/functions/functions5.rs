// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// 计算数字的平方：接收i32类型参数，返回其平方值（i32类型）
fn square(num: i32) -> i32 {
    num * num
}
