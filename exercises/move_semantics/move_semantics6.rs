fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用，不转移所有权

    string_uppercase(data); // 传递所有权
}

// 不获取所有权，接收引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 获取所有权，接收String
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // to_uppercase返回新字符串，重新赋值给data
    println!("{}", data);
}
