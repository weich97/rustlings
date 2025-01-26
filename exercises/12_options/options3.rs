#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(Point { x, y }) => println!("Co-ordinates are {},{}", x, y),
        None => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
