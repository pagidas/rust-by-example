# rust-by-example
A repository to track progress on learning Rust.

> [!NOTE]
> The official [guide](https://doc.rust-lang.org/rust-by-example/index.html) we're following.

# Structure
The repository is broken down into multiple modules built in `cargo` -- Rust's *package manager*. 

We've tried to keep the original structure of the ***rust-by-example*** guide linked above.

## Modules
<details>
<summary>1. Hello World</summary>

- [hello-world program](./1-hello-world/hello-world/)
- [Formatted print](./1-hello-world/formatted-print/)
- [Formatted Debug](./1-hello-world/formatted-debug/)
- [Formatted Display](./1-hello-world/formatted-display/)
    - [Testcase: List](./1-hello-world/formatted-display/src/main.rs#L32-L52)
</details>

<details>
<summary>2. Primitives</summary>

- [Primitives](./2-primitives/primitives)
- [Literals and Operators](./2-primitives/literals-and-operators/)
- [Tuples](./2-primitives/tuples)
- [Arrays and Slices](./2-primitives/arrays-and-slices/)
</details>

<details>
<summary>3. Custom Types</summary>

- [Structures](./3-custom-types/structures/)
    - [Testcase: Rectangle area](./3-custom-types/structures/src/main.rs#L72-L79)
- [Enums](./3-custom-types/enums/)
    - [Definition](./3-custom-types/enums/definition/)
    - [Use](./3-custom-types/enums/use-scoping/)
    - [C-like](./3-custom-types/enums/c-like/)
    - [Testcase: linked-list](./3-custom-types/enums/testcase-linked-list/)
- [Constants](./3-custom-types/constants/)
</details>

<details>
<summary>4. Variable Bindings</summary>

- [Binding](./4-variable-bindings/binding/)
- [Scope and Shadowing](./4-variable-bindings/scope-and-shadowing/)
- [Declare first](./4-variable-bindings/declare-first/)
- [Freezing](./4-variable-bindings/freezing/)
</details>

<details>
<summary>5. Types</summary>

- [Casting](./5-types/casting)
- [Literals](./5-types/literals)
- [Inference](./5-types/inference)
- [Aliasing](./5-types/aliasing)
</details>

<details>
<summary>6. Conversion</summary>

- [From and Into](./6-conversion/from-and-into)
- [TryFrom and TryInto](./6-conversion/tryfrom-and-tryinto)
- [To and from Strings](./6-conversion/to-and-from-strings)

</details>

<details>
<summary> 7. Expressions</summary>

- [expressions](./7-expressions/expressions)

</details>
