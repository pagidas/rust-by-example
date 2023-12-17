# Notes

### Takeaways
- `enum`s can have have **methods** attached to them, using the `impl` construct
```rust
enum Work {
    Civilian,
    Solidier,
}

impl Work {
    // Conumes a `Work` and prints text to the console.
    fn inspect(&self) {
        match self {
            Self::Civilian => println!("Civilians work!")
            Self::Solidier => println!("Solidiers fight!")
        }
    }
}

fn main() {
    let civilian = Work::Civilian;
    civilian.inspect();
}
```

### Questions
