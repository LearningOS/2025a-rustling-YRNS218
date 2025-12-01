// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let vec0 = Vec::new();

    // 修复：通过引用传递，避免所有权转移
    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 修复：接收引用并返回新的 Vec，不获取原 Vec 的所有权
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // 创建一个新的 Vec，复制原 Vec 的内容（如果有的话）
    let mut vec = vec.clone();
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
