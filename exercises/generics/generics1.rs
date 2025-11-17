// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    // 修复：指定 Vec 的泛型类型为 &str（字符串切片），与后续 push 的值类型匹配
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
