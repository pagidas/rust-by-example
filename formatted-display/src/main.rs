use std::fmt;

// A strcuture holding two numbers
#[derive(Debug)]
struct MinMax(i64, i64);

// `Display` implementation
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// A structure where its fields are nameable.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

// `Display implementation`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Testcase_start: List
struct List(Vec<i32>);

// Prints the list indexed separated by colon.
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // extract the value of the type, using tuple indexing,
        // and create a reference to `vec`.
        let vec: &Vec<i32> = &self.0;

        write!(f, "[")?;

        // iterate over `vec` while enumrating the iteration count in `index`.
        for (index, v) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{index}: {value}", index = index, value = v)?;
        }

        // close the opened bracket and return fmt::Result value.
        write!(f, "]")
    }
}
// printing the `List(Vec<i32>)` is used in main towards the end.
// Testcase_end: List

fn main() {
    let minmax: MinMax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big}, and the small is {small}",
            small = small_range,
            big = big_range);

    let point: Point2D = Point2D { x: 3.3, y: 7.2 };
    
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Testcase: List, Display");
    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}
