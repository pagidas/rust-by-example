// A named struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32
}

// Structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Kostas");
    let age = 32;
    let kostas = Person { name, age };

    // Print debug struct
    println!("{:?}", kostas);

    // Instantitate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates ({}, {})", point.x, point.y);

    // Make a new point by using update syntax to use the fields of our other one
    let bottom_right: Point = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle: Rectangle = Rectangle { 
        top_left: Point { x: left_edge, y: top_edge }, 
        bottom_right: bottom_right 
    };

    // Instantiate a `Unit` struct
    let unit = Unit;

    // Instantitate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("destructured pair contains {:?} and {:?}", integer, decimal);

    println!("area of the rectangle is: {}", rect_area(_rectangle));
}

fn rect_area(given_rectangle: Rectangle) -> f32 {
    let Rectangle { 
        top_left: Point { x: x1, y: y1 }, 
        bottom_right: Point { x: x2, y: y2 } 
    } = given_rectangle;

    return (x1 - x2) * (y1 - y2);
}
