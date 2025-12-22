
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<(u32)> = None;
    if my_option.is_none() {
       println!("Option is None");
    }

    let my_arr = [
        -1, -2, -3
        -4, -5, -6
    ];
    println!("Array: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 42;
    let mut value_b = 1337;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value_a: {}, value_b: {}", value_a, value_b);
}
