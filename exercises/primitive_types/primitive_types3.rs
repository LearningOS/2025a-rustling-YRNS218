// primitive_types3.rs
//
//Create an array with at least 100 elements in it where the ??? is.
//
//Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
//for a hint.

fn main() {
    let a = [0; 100]; // 使用重复语法创建包含100个0的数组
    // 也可以用 let a = [0; 101]; 等超过100的长度

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
