# Notes

### Takeaways
Ambiguous types such as *generic containers*, `Vec<T>`, cannot be displayed using `fmt::Display` trait since `std` library does not provide an implementation. Rather, `fmt::Debug` should be used.

`write!` returns an `fmt::Result` which is a boxed type that contains two variants: `Ok` ***or*** `Err`. To deal with multiple results, Rust provides the `?` operator which can be used at the end of an expression (which returns the `Result`) that expands the `Err(err)` variant to an early return.

Example:
```rust
// returns immediately if `Err`, else `Ok` expression.
write!(f, "{}", value)?;
```

### Questions
