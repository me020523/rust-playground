fn main() {
    println!("hello, world")
}

use std::cmp::PartialOrd;
fn max<T: PartialOrd>(list: &[T]) -> &T {
    let mut ret = &list[0];
    for i in list {
        if i >= ret {
            ret = i
        }
    }
    ret
}

struct Point<T,S> {
    a: T,
    b: S,
}

impl<T> Point<T, f32> {
    fn show_f32(&self) {
        println!("f32 is {}", self.b);
    }
}

impl<S> Point<u8,S> {
    fn show_u8(&self) {
        println!("u8 is {}", self.a);
    }
}

fn run_generic_1() {
    let p1 = Point{
        a: String::from("hello,world"),
        b: 1.4f32,
    };
    p1.show_f32();

    let p2 = Point{
        a: 10u8,
        b: String::from("hello,world"),
    };
    p2.show_u8();
}
