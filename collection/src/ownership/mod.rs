fn run() {

}

struct Point {
    a: i8,
    b: i8,
}
fn vec_ownship() {
    let points := Vec::new();
    points.push(Point{a:1, b:2});
    points.push(Point{a:1, b:2});

    let p = points[0]; //默认使用borrow

    println!(points[0]);
}
