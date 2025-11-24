// strings4.rs
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // "blue"是字符串字面量，类型为&str
    string("red".to_string()); // to_string()返回String
    string(String::from("hi")); // String::from创建String
    string("rust is fun".to_owned()); // to_owned()为&str创建String
    string("nice weather".into()); // into()将&str转换为String（类型推断）
    string(format!("Interpolation {}", "Station")); // format!返回String
    string_slice(&String::from("abc")[0..1]); // 切片操作返回&str
    string_slice("hello there".trim()); // trim()返回&str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace()返回String
    string("my SHIFT key IS stuck".to_lowercase()); // to_lowercase()返回String
}
