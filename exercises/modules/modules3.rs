
// 引入 std::time 模块中的 SystemTime 和 UNIX_EPOCH（一行完成）
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 到现在经过了 {} 秒", n.as_secs()),
        Err(_) => panic!("时间早于UNIX纪元！"),
    }
}
