pub mod shape;
pub mod line;

use shape::Rect;
use shape::Circle;
use line::Line;
use shape::Shape;

pub fn run() {
    let r = Rect{
        width: 10.0,
        height: 5.0,
    };
    println!("rect area is {}", r.area());

    let c = Circle{r: 10.0};
    println!("circle area is {}", c.area());

    let l = Line{
        begin: 10.0,
        end: 20.0,
    };
    println!("line area is {}", l.area());
}
