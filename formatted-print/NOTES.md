# Notes

### Takeaways

#### Printing
It is handled by a series of `macros` defined in `std::fmt` some of which include:
- `format!`: write formatted text to `String`.
- `print!`: sames as `format!` but the text is printed in the console (io::stdout).
- `println!`: same as `print!` but a newline is appended.
- `eprint!`: same as `print!` but the text is printed to the standard error (io::stderr).
- `eprintln!`: same as `eprint!` but a newline is appended.

#### std::fmt
`std::fmt` contains `traits` (interfaces) which govern the display of text.
- `fmt::Display`: Uses the `{}` marker. Format text in a more user-friendly fashion.
- `fmt::Debug`: Uses the `{:?}` marker. Format text for debugging puposes.

> **Note** <br>
The standard library provides implementations for types in the standard library. For custom types, we have to implement the `fmt::Display` that demands us to implement the `ToString` trait as well, so that we can convert the type to `String`.

### Questions
