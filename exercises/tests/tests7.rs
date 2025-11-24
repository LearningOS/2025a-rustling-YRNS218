//! 该构建脚本用于设置环境变量 TEST_FOO，供 tests7.rs 测试使用

fn main() {
    // 获取当前 Unix 时间戳（秒级）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 向 Cargo 输出指令，设置环境变量 TEST_FOO
    // 变量值为当前时间戳，确保在测试的时间范围内
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
}
