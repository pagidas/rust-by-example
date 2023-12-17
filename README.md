# rust-by-example
A repository to track progress on learning the basics in Rust.

> [!NOTE]
> The official [guide](https://doc.rust-lang.org/rust-by-example/index.html) we're following.

# Structure
The repository is broken down into multiple modules, built in `cargo` -- Rust's *package manager*. 

We've tried to keep the original structure of the ***rust-by-example*** guide -- linked mentioned above.

## Modules
1. [Hello World](./1-hello-world/)
    - [hello-world program](./1-hello-world/hello-world/)
    - [Formatted print](./1-hello-world/formatted-print/)
    - [Formatted Debug](./1-hello-world/formatted-debug/)
    - [Formatted Display](./1-hello-world/formatted-display/)
        - [Testcase: List](./1-hello-world/formatted-display/src/main.rs#L32-L52)
2. [Primitives](./2-primitives)
    - [Primitives](./2-primitives/primitives)
    - [Literals and Operators](./2-primitives/literals-and-operators/)
    - [Tuples](./2-primitives/tuples)
    - [Arrays and Slices](./2-primitives/arrays-and-slices/)
3. [Custom Types](./3-custom-types/)
    - [Structures](./3-custom-types/structures/)
        - [Testcase: Rectangle area](./3-custom-types/structures/src/main.rs#L72-L79)
    - [Enums](./3-custom-types/enums/)
        - [Definition](./3-custom-types/enums/definition/)
        - [Use](./3-custom-types/enums/use-scoping/)
