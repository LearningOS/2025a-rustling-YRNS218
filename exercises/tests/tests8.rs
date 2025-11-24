//! 该构建脚本同时满足 tests7.rs 和 tests8.rs 的需求
//! 1. 为 tests7 设置环境变量 TEST_FOO
//! 2. 为 tests8 启用 "pass" 特性

fn main() {
    // 1. 处理 tests7：设置环境变量 TEST_FOO（值为当前 Unix 时间戳）
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 2. 处理 tests8：启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
