fn main() {
    let x = 5u32;

    // block as an expression terminated by ; makes a single statement
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // this expression will be assigned to `y`
        x_cubed + x_squared + x
    };

    // block as an expression
    let z = {
        // the ; suppreses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
