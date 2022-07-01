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



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn can_hold(one:&Rectangle, other: &Rectangle) -> bool {
        one.width > other.width && one.height > other.height
    }

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(can_hold(&smaller,&larger));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!can_hold(&larger,&smaller));
        
        
    }


