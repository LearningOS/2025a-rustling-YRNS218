fn main() {
    let mut vec0 = Vec::new();

    // 直接将 vec0 传入 fill_vec（转移所有权），处理后再取回
    vec0 = fill_vec(vec0);

    // 此时 vec0 已被 fill_vec 填充内容
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    // 将 vec0 的内容克隆到 vec1，再修改 vec1
    let mut vec1 = vec0.clone();
    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

// 接收 Vec 的所有权，填充内容后返回
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
