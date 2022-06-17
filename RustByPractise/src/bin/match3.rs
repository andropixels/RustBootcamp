
struct Point {
    x: i32,
    y: i32,
}
// The @ operator lets us create a variable that holds a value,
// at the same time we are testing that value to see whether it matches a pattern


fn main() {
    // Fill in the blank to let p match the second arm
    let p = Point { x: __, y: __ };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
