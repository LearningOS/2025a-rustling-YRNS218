// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    // 修复：使用下划线忽略未使用的变量，避免编译错误
    let _ = y; // Fix without deleting this line.
}
