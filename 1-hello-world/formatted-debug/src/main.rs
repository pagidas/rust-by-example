#[derive(Debug)]
struct Structure(i32); // a simple struct containing an i32

#[derive(Debug)]
struct Deep(Structure); // a complex struct containing another struct

fn main() {
    // Printint basic standard types on Debug.
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor = "actor's");
    
    // Printing `Structure` on Debug.
    println!("Now {:?} will print!", Structure(3));

    // Printing `Deep` on Debug.
    // "Pretty printing" with `#` in {:#?}.
    println!("Now {:#?} will print!", Deep(Structure(15)))
}
