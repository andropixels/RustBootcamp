#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
#[allow(dead_code)]
struct Nil;

// A tuple struct
#[allow(dead_code)]
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name: name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point = Point { x: 0.3, y: 0.4 };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle { p1: point, p2: Point { x: 1.0, y: 1.0 } };

    let a = rect_area(&rect);

    println!("area of {rect:?} is {a}", rect=rect, a=a);

    let b = square(Point {x: 0.5, y: 0.5}, 1.3);

    println!("square {:?}", b);   
}

fn rect_area(rect: &Rectangle) -> f32 {
    let &Rectangle { 
        p1: Point { x: x1, y: y1 }, 
        p2: Point { x: x2, y: y2 }
    } = rect;

    return (x2 - x1).abs() * (y2 - y1).abs();
}

fn square(origin: Point, dimension: f32) -> Rectangle {
    let Point {
        x, y
    } = origin;

    Rectangle{ p1: origin, p2: Point { x: x + dimension, y: y + dimension}}
}