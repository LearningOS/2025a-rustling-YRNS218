#[rustfmt::skip]
macro_rules! my_macro {
    // 分支之间添加逗号分隔
    () => {
        println!("Check out my macro!");
    },
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
