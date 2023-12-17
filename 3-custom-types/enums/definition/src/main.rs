// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or `c-like` structs.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y)
    };
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let pressed: WebEvent = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted : WebEvent = WebEvent::Paste("my text".to_owned());
    let click: WebEvent = WebEvent::Click { x: 20, y: 80 };
    let load: WebEvent = WebEvent::PageLoad;
    let unload: WebEvent = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // We can refer to each variant via its alias, not its long and inconvenient name.
    let add = Operations::Add;
    let subtract = Operations::Subtract;

    println!("Sum of 1 and 2 equals {}", add.run(1, 2));
    println!("Subtract of 2 and 1 equals {}", subtract.run(2, 1));
}
