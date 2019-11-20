struct Point {
    a: i8,
    b: i8
}

pub fn run() {
    println!("----ref keyword case----");
    mutability();
    ref_field();
    println!("----done----")
}

fn mutability() {
    let mut p = Point{a:1,b:2};
    println!("p.a is {}", p.a);
    let ref q = p;
    //q.a = 1; //compiler panic, q is a immutable reference
    println!("new p.a is {}", q.a);

    let ref mut q2 = p;
    q2.a = 1;
    println!("new p.a is {}", p.a);

    let r = Point{a:2,b:3};
    let mut q = r;
    q.a = 4;
    println!("q.a is {}", q.a);
}

fn ref_field() {
    let p = Point{a:1,b:2};
    let Point{a: to_a, b: _} = p;
    println!("p.a is {}", to_a);
    //ref_to_a = 10;   //compiler panic, ref_to_a is a immutable reference

    let Point{a: mut to_a2, b: _} = p;
    println!("p.a is {}", to_a2);
    to_a2 = 10;
    println!("p.a is {}", to_a2);
}
