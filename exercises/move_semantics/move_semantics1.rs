fn main() {
    let vec0 = Vec::new();

    // 通过引用传递 vec0，避免所有权转移
    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 接收原 Vec 的不可变引用，返回新的 Vec（不获取原 Vec 所有权）
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // 克隆原 Vec 的内容，创建新的可变 Vec
    let mut new_vec = vec.clone();
    // 向新 Vec 中添加元素
    new_vec.push(22);
    new_vec.push(44);
    new_vec.push(66);
    // 返回新 Vec
    new_vec
}
