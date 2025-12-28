// modules2.rs
// The delicious_snacks module is trying to present an external interface that is
// different than its internal structure (the `fruits` and `veggies` modules and
// associated constants). Complete the `use` statements to fit the uses in main and
// find the one keyword missing for both constants.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.

mod delicious_snacks {
    // TODO: 完成 use 语句
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}