fn main() {
    // 直接调用fill_vec，不再提前创建vec0
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// fill_vec不再接收参数，内部自己创建Vec并填充内容
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
