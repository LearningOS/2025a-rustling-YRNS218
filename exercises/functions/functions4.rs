
fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

// 计算促销价格：接收i32类型价格，返回i32类型促销价
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

// 判断数字是否为偶数：接收i32类型数字，返回bool类型结果
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
