#[cfg(test)]
mod tests {
    #[test]
    fn test_unsafe() {
        let mut address = 0x12345678u32;
        unsafe {
            *(&mut address as *mut u32) = 0xAABBCCDD;
        }
        assert_eq!(address, 0xAABBCCDD);
    }
}