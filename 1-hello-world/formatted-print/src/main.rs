fn main() {
    // The `{}` will be automatically replaced with any arguments. These will be stringified
    println!("{} days", 31);
    
    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Bob", "Alice");

    // Named arguments
    println!("{subject}, {verb}, {object}",
            subject="the quick brown fox",
            verb="jumps over",
            object="the lazy dog");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will prepend 4 white spaces and a "1", for a total width of 5.
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    struct Structure(i32); // compiles but warns that the struct is never constructred.
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
}
