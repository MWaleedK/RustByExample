use std::fmt;

struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} ,y: {}", self.x, self.y)
    }
}

impl fmt::Binary for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b} + {:b}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

fn main() {
    let minmax = MinMax(0, 15);
    println!("Compare Structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    println!("Binary {:b}", minmax); // Done

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 2.2, y: 5.4 };
    println!("Compare Points");
    println!("Debug {}", point);
    println!("Display {:?}", point);

    let complex = Complex {
        real: 2.2,
        imaginary: 5.4,
    };
    println!("Compare complexs");
    println!("Debug {}", complex);
    println!("Display {:?}", complex);
}
