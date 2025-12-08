fn main() {
    let mut x = 100;
    
    // 先使用 y 的可变引用，用完后再创建 z 的可变引用
    let y = &mut x;
    *y += 100;

    // y 的作用域结束后，再创建 z 的可变引用
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
