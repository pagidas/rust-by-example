# Notes

### Takeaways
- The `use` declaration can be utilized to avoid **manual scoping** in `enums`

```rust
// Given
enum Work {
    Civilian,
    Solidier,
}

// We can
fn main() {
    use crate::Work::{Civilian, Solidier};
    // Or `crate::Work::*;` if we want to include all variants.
    
    // Equivalent to `Status::Civilian`.
    let work: Work = Civilian;
}
```

### Questions
