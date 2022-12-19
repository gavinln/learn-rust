#[allow(unused)]

fn main() {
    let x = 5 as u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x // will be assiged to x
    };

    let z = {
        2 * x; // () assigned to z
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
