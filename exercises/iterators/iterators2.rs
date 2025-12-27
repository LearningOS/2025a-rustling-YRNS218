
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            // 将首字母大写，拼接剩余字符
            let mut result = first.to_ascii_uppercase().to_string();
            result.push_str(c.as_str());
            result
        }
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 迭代每个单词，应用 capitalize_first 并收集为 Vec
    words.iter().map(|&word| capitalize_first(word)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    // 迭代每个单词，应用 capitalize_first 后拼接为单个字符串
    words.iter().map(|&word| capitalize_first(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
