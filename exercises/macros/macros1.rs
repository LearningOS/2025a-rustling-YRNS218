macro_rules! my_macro {
    // 宏分支的语法：模式 => { 代码块 }; （代码块内的语句要以分号结尾）
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
