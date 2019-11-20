use super::shape::Shape;

pub struct Line {
    pub begin: f32,
    pub end: f32,
}

impl Shape for Line {
    fn area(&self) -> f32 {
        self.end-self.begin
    }
}
