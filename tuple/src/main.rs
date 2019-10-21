fn main() {
    let tup: (i32, f64, u8) = (123, 1.1, 1);

    let (x, y, z) = tup;

    println!("x is {}", x);
    println!("tup.0 is {}", tup.0);
    println!("y is {}", y);
    println!("tup.1 is {}", tup.1);
    println!("z is {}", z);
    println!("tup.2 is {}", tup.2);
}
