fn main() {
    // Scoping
    // this binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short {}", short_lived_binding);
    }
    // End of block
    
    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long {}", long_lived_binding);
    // End Scoping

    // Shadowing
    let shadowed_binding = 1;

    {
        println!("before being shadowed {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("after being shadowed {}", shadowed_binding);
    }
    println!("outside inner block {}", shadowed_binding);

    // This binding *shadows* the previous one
    let shadowed_binding = 2;

    println!("shadowed in outer block {}", shadowed_binding);
    // End Shadowing
}
