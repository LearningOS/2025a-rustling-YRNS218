use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cicverify() {
    // 1. 子命令（cicverify）要放在参数列表的第一个位置
    // 2. args 接收的是 &[&str] 类型的切片，修正语法格式
    Command::cargo_bin("rustlings")
       .unwrap()
       .args(&["cicverify", "--nocapture"]) // 顺序：先子命令，后参数
       .assert()
       .success();
}
