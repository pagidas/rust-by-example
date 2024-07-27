# Notes

### Takeaways
`From` and `Into` traits are inherentely linked. If we can conver type `A` from type `B`, then it should be easy to convert type `B` to type `A`.

#### From
- It allows for type to define how to create itself from another type

#### Into
- It defines how to convert a type into another one
- Calling `int()` requires to specify the result type as the compiler is unable to determine this most of the time

#### Inherentely linked
`From` and `Into` are designed to be complementary. If you have implemented the `Form` trait for your type, `Into` will call it when necessary. However, the opposite **is not** true.

### Questions
