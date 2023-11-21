# Notes

### Takeaways
Ambiguous types such as *generic containers*, `Vec<T>`, cannot be displayed using `fmt::Display` trait since `std` library does not provide an implementation. Rather, `fmt::Debug` should be used.

### Questions
