fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // 直接接收fill_vec返回的Vec（原vec0的所有权转移后被填充）

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> { // 接收Vec所有权，填充后返回
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
