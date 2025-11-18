// modules2.rs
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.

mod delicious_snacks {
    // 引入 fruits 模块中的 PEAR 并别名为 fruit
    use self::fruits::PEAR as fruit;
    // 引入 veggies 模块中的 CUCUMBER 并别名为 veggie
    use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }

    // 暴露别名供外部使用（通过 pub use 重导出）
    pub use self::fruit;
    pub use self::veggie;
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
