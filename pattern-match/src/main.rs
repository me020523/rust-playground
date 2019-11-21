fn main() {
    match_literal();
    match_enum();
    match_struct();
}

fn match_literal() {
    let x = 10;
    print!("literal matching:");
    match x {
        1 => {
            println!("{}", 1);
        }
        2 => {
            println!("{}",2);
        }
        _ => {
            println!("ignored");
        }
    }
}

enum Color {
    Green(u32),
    Red(u32),
    Yellow(char),
}

enum Fruit {
    Apple(Color),
    Pear(u32),
    Banana{c:Color, s: u32},
}

fn match_enum() {
    let apple = Fruit::Apple(Color::Green(10));
    let pear = Fruit::Pear(10);
    let banana = Fruit::Banana{c:Color::Yellow('y'), s:10};
    let _red_apple = Fruit::Apple(Color::Red(4));

    let t = [apple, pear, banana];
    let mut i = 0;
    while i < t.len() {
        print!("match enum:  ");
        match &t[i] {
            Fruit::Apple(Color::Red(4)) => {
                println!("found a red apple");
            }
            Fruit::Apple(Color::Red(x)) => {
                println!("found an apple, the color is {}",x);
            }
            Fruit::Apple(_) => {
                println!("is an apple")
            }
            Fruit::Pear(s) if *s > 11 => {
                println!("found an pear, its size is greater than 11");
            }
            Fruit::Pear(s) if *s <= 11 => {
                println!("found an pear, its size is not greater than 11");
            }
            Fruit::Pear(_) => {
                println!("found an pear");
            }
            Fruit::Banana{c: ref _bc @ Color::Yellow('y'), s} if *s <= 11 => {
                println!("found a banana, color is bc, size is {}", s);
            }
            Fruit::Banana{c:_,s} => {
                println!("found a banana, color is c, size is {}",  s);
            }
        }
        i += 1;
    }
}

struct Point {
    x : u32,
    y : u32,
}
fn match_struct() {
    let p = Point{x: 10, y:16};
    match p {
        Point{x: 10, y} if y >= 17 => {
            println!("struct match: y>=17 && x = 10");
        }
        Point{x: 10, y} if y < 17 => {
            println!("struct match: y<17 && x=10");
        }
        Point{x:_, y: new_y @ 5..=10} => {
            println!("struct match: 5<=y<=10, y is {}", new_y)
        }
        Point{x,y} => {
            println!("struct match: x is {}, y is {}", x,y);
        }
    }
}
