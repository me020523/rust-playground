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
    let s3 = s2.clone();
    println!("{},{}, world!", s2,s3);

    test_fn_argument(s3);
    //println!("{},world!", s3);

    test_borrow_change(&mut s3)
}

fn test_fn_argument(s:String) {
    println!("{},world!", s);
}

fn test_borrow_change(s:& mut String) {
    s.push_str(", world");
}
