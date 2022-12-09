// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f64 {
    let width = r.bottom_right.x - r.top_left.x;
    let height = r.bottom_right.y - r.top_left.y;
    (width * height).abs()
}

fn square(p: Point, length: f64) -> Rectangle {
    let width = p.x + length;
    let height = p.y + length;
    let r = Rectangle {
        top_left: p,
        bottom_right: Point {
            x: width,
            y: height,
        },
    };
    return r;
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { y: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    //let Point {
    //    x: left_edge,
    //    y: top_edge,
    //} = point;
    let (left_edge, top_edge) = (point.x - 5.0, point.y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let area = rect_area(_rectangle);
    println!("Double area of _rectangle = {}", area * 2.0);

    let sqr = square(Point { x: 1.0, y: 1.0 }, 4.0);
    println!("Area of square = {}", rect_area(sqr));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
