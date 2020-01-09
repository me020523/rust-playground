fn main() {
    let mut x = 5;
    println!("the value of x is: {}",x);
    x = 6;
    println!("the value of x is: {}", x);
}

fn test_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1);
    let mut s3 = s2.clone();
    println!("{},{}, world!", s2,s3);
    //test_fn_argument(s3);
    //println!("{},world!", s3);

    let s4 = &mut s3;
    let c1 = s4.len();
    println!("c1 is {}", c1);

    let s5 = &mut s3;
    let c2 = s5.len();
    println!("c2 is {}", c2);

    test_borrow_change(&mut s3)
}

fn test_fn_argument(s:String) {
    println!("{},world!", s);
}

fn test_borrow_change(s:& mut String) {
    s.push_str(", world");
}
