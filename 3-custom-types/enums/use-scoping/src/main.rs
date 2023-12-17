enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Solidier,
}

fn main() {
    // Explicitly `use` each name so they are available without manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each variant inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;

    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money...")
    }

    match work {
        Civilian => println!("Civilians work!"),
        Solidier => println!("Solidiers fight!"),
    }
}
