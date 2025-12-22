
use std::f32;
fn main() {
    let pi = 3.14f32;
    let radius: f32= 5.0;

    let area =pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}
