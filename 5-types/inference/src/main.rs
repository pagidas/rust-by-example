fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // Create an empty vector (growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`)
    
    // Insert `elem` in the vector
    vec.push(elem);
    // The compiler now knows that `vec` is of type `Vec<u8>` because it also checks that vector
    // inserted such an type

    println!("{:?}", vec);
}
