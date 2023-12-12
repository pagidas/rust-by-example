# Notes

### Takeaways
A sample of Rust's primitive types.

#### Scalar Types
- **Signed integers**: `i8`, `i32`, `i64`, `i128` and `isize` (pointer size)
- **Unsigned integers**: `u8`, `u32`, `u64`, `u128` and `usize` (pointer size)
- **Floating point**: `f32`, `f64`
- `char` Unicode scalar values like `'a'`, `'α'`, `'∞'` (4 bytes each)
- `bool` either `true` or `false`
- The unit type `()`, whose only possible value is an empty tuple: `()`

> [!NOTE]
> *Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values*.

#### Compound Types
- Arrays like `[1, 2, 3]`
- Tuples like `(1, true)`

> [!NOTE]
> *Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context*.

Using the `mut` keyword we can declare **mutable** variables which otherwise are **immutable** by default.

### Questions
