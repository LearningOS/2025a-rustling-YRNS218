// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using AsMut.
fn num_sq<T: AsMut<u32>>(arg: T) {
    // Dereference the arg to get the value, square it, then store it back
    *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_eq!(byte_counter(s), 11);
        assert_eq!(char_counter(s), 10);
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(byte_counter(s), 10);
        assert_eq!(char_counter(s), 10);
    }

    #[test]
    fn num_sq_test() {
        let mut x = 5u32;
        num_sq(&mut x);
        assert_eq!(x, 25);
    }
}