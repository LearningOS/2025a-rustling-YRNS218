// from_into.rs
//
// The trait `From` is used for value-to-value conversions. If `From` is
// implemented correctly for a type, the `Into` trait should work conversely. You
// can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

use std::convert::From;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// 实现 From<&str> 来解析 "Name, Age" 格式的字符串
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 按逗号分割，去掉每个部分的前后空格
        let parts: Vec<&str> = s.split(',').map(|part| part.trim()).collect();
        
        // 处理名字：取第一个非空部分，默认是 "Mike"
        let name = parts.get(0)
            .filter(|&part| !part.is_empty())
            .cloned()
            .unwrap_or("Mike")
            .to_string();

        // 处理年龄：取第二个部分解析，失败则默认 0
        let age = parts.get(1)
            .and_then(|part| part.parse().ok())
            .unwrap_or(0);

        Person { name, age }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let p = Person::from("");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("John, 30");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("John");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_trailing_comma() {
        let p = Person::from("John, 30,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p = Person::from("John, 30, blah");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p = Person::from(", 30");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p = Person::from("John,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p = Person::from(",");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p = Person::from(", blah");
        assert_eq!(p.name, "Mike");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p = Person::from("John");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 0);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("John, blah");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 0);
    }
}