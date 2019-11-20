pub trait Shape {
    fn area(&self) -> f32;
}

pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Circle {
    pub r: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14*self.r
    }
}
