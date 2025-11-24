// traits2.rs
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
// No boiler plate code this time, you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为Vec<String>实现AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // 创建一个新的向量，克隆原向量的所有元素
        let mut new_vec = self;
        // 向新向量中添加字符串"Bar"
        new_vec.push(String::from("Bar"));
        // 返回更新后的向量
        new_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
