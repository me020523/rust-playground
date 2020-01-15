use std::cmp::Ordering;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle{
            width:0,
            height: 0,
        }
    }

    fn area(&self) -> u32 {
        self.height*self.width
    }
}

impl PartialEq for Rectangle {
    fn eq(&self,rect: &Rectangle) -> bool {
        return self.area() == rect.area();
    }
}
impl PartialOrd for Rectangle {
    fn partial_cmp(&self, rect: &Rectangle) -> Option<Ordering> {
        let a1 = self.area();
        let a2 = rect.area();
        return Some(a1.cmp(&a2))
    }
}

fn main() {
    play_meta();
    play_trait();
}

fn play_meta() {
    let mut rect = Rectangle::new();
    rect.width = 10;
    rect.height = 10;

    let mut rect2 = Rectangle::new();
    rect2.width = 9;
    rect2.height = 8;

    assert_eq!(rect>rect2, true);
    println!("the area is {}", rect.area());
}
mod mytrait;
fn play_trait() {
    mytrait::run();
}
