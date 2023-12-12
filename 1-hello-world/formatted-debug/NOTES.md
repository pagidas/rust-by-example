# Notes

### Takeaways
Since there is a default implementation for the `fmt::Debug` `trait` supported in the standard library, *all* types can `derive` (automatically create) it. To do so we have to decorate the type with the `derive` attribute.
```rust
#[derive(Debug)]
struct Structure(i32)
```

### Questions
