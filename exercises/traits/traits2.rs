
trait AppendBar {
    fn append_bar(self) -> Self;
}
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_vec = self;
        new_vec.push(String::from("Bar"));
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
