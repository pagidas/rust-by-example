# Notes

### Takeaways
>*Rust has two types of constants which can be declared in any scope including global. Both require explicit annotation*
- `const`: An unchangeable value (the common case).
- `static`: A possibly `mut`able variable with `static` lifetime. The static lifetime in inferred and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`.

### Questions
