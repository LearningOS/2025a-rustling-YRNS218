use console::{style, Emoji};
use std::env;

// 定义 warn 宏：输出红色警告信息（可选关闭 emoji）
macro_rules! warn {
    ($fmt:literal, $($ex:expr),*) => {{
        let formatstr = format!($fmt, $($ex),*);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("!").red(), style(formatstr).red());
        } else {
            println!(
                "{} {}",
                style(Emoji("⚠️ ", "!")).red(),
                style(formatstr).red()
            );
        }
    }};
}

// 定义 success 宏：输出绿色成功信息（可选关闭 emoji）
macro_rules! success {
    ($fmt:literal, $($ex:expr),*) => {{
        let formatstr = format!($fmt, $($ex),*);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("✓").green(), style(formatstr).green());
        } else {
            println!(
                "{} {}",
                style(Emoji("✅ ", "✓")).green(),
                style(formatstr).green()
            );
        }
    }};
}

// 示例：调用宏
fn main() {
    warn!("配置文件不存在，请检查路径");
    success!("数据同步完成，共处理 {} 条记录", 128);
}
