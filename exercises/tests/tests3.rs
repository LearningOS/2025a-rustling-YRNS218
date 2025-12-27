
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4)); // 测试偶数（如4）调用is_even应返回true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // 测试奇数（如5）调用is_even应返回false
    }
}
