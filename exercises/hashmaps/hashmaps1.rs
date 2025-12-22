
// 结构体泛型化：使用类型参数 T 替代具体类型 u32
struct Wrapper<T> {
    value: T,
}

// 实现泛型化：为所有 T 类型实现 Wrapper<T> 的方法
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
