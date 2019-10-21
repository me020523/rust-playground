fn main() {
    leftValue();
    rightValue();
}

fn leftValue() {
    let x = 5;
    //x = 6; //compiler panic
    let mut y = 5;
    println!("x is {}, y is {}", x, y);
    y = 6;
    println!("new y is {}", y)
}

fn rightValue() {
    struct Point {
        pub a : i8,
        pub b : i8
    }

    let t = Point{a: 1, b:2};
    let s = &t;
    //s.a = 3; //compiler panic
    println!("{},{}", s.a, s.b);

    let mut p = Point{a:1, b:2}; //does need to be mutable
    let q = &p;
    //q.a = 10; //compiler still panic
    println!("{},{}", q.a, q.b);

}

fn scalarValue() {
    //let x = 5; //compiler panic
    let mut x = 5;
    println!("x is {}", x);
    let y = &mut x;
    *y = 6;
    println!("x is {}", x)
}
