// clippy1.rs
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

use std::f32::consts::PI;

fn main() {
    let pi = PI;
    let radius = 5.0f32;
    let area = pi * radius * radius;

    println!("Area of a circle with radius {} is {}", radius, area);
}