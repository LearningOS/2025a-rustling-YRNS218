

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 使用Box包裹List解决递归类型大小不确定的问题
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 空列表直接返回Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))) // 非空列表：1 -> 2 -> Nil
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
