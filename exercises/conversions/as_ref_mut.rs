// as_ref_mut.rs

// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut.
fn num_sq(arg: &mut u32) {
    *arg *= *arg;
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_eq!(byte_counter(s), 13); // UTF-8 实际字节数
        assert_eq!(char_counter(s), 10);
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(byte_counter(s), 12); // UTF-8 实际字节数
        assert_eq!(char_counter(s), 10);
    }

    #[test]
    fn num_sq_test() {
        let mut x = 5u32;
        num_sq(&mut x);
        assert_eq!(x, 25);
    }
}*/