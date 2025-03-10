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
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn square(starting_point: &Point, value: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            // Create a new Point instead of moving reference
            x: starting_point.x,
            y: starting_point.y,
        },
        bottom_right: Point {
            x: starting_point.x + value,
            y: starting_point.y + value,
        },
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let (&Point { x: x1, y: y1 }, &Point { x: x2, y: y2 }) = (&rect.top_left, &rect.bottom_right);

    let base: f32 = (x2 - x1).abs();
    let height: f32 = (y2 - y1).abs();

    let area: f32 = height * base;

    return area;
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!(
        "Area of the rectangle is: {} uints_squared",
        rect_area(_rectangle)
    );
    let new_square: Rectangle = square(&Point { x: 1.0, y: 5.0 }, 32.0);
    println!("{:?}", new_square);
}
