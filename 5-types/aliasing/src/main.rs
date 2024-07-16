// `Nanosecond`, `Inch` and `U64` are new names for `u64`
type Nanosecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `Nanosecond` = `Inch` = `U64` = `u64`
    let nanoseconds: Nanosecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Note that type aliases don't provide any
    // extra type safety, becauses they're not
    // a new type
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
