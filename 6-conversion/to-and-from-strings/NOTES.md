# Notes

### Takeaways

#### Converting to String
To convert any type to a `String` is as simple as implementing the `ToString` trait for the type. Rather than doing this directly, we can implement the `fmt::Display` trait which automagically provides `ToString`.

#### Parsing a String
The idiomatic approach to parse from a `String` is to use the `parse()` and infer the expected type in the variabla binding or use the 'turbofish' syntax. 
- `let parsed: i32 = "5".parse().unwrap();`
- `let parsed = "10".parse::<i32>().unwrap();`

The given type will be converted to `String` as long as it implementes the `FromStr` trait. Many of types in the language already provide an implementation as such.

### Questions
