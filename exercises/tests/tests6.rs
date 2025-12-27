

struct Foo {
    a: u128,
    b: Option<String>,
}

unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });
        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };
        let ptr_2 = &ret.a as *const u128 as usize;
        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
