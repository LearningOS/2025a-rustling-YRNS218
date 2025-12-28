// clippy3.rs
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_option: Option<u32> = None;
    if let Some(x) = my_option {
        println!("{}", x);
    }

    let my_array = [
        -1, -2, -3,
    ];
    println!("{:?}", my_array);
}