mod macros {
    // 用 #[macro_export] 导出宏（让模块外可以使用）
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // 直接使用导出的宏（因为 #[macro_export] 会将宏导入到 crate 根作用域）
    my_macro!();
}
