mod test1 {
    pub fn hello() {
        println!("{}", "test1.hello");
    }
}

mod opoverride {
    use std::ops::Add;

    pub struct MyStruct {
        pub a : u32,
        pub b : u32,
    }

    impl Add<MyStruct> for MyStruct {
        type Output = Self;
        fn add(self, o: Self) -> Self::Output {
            Self{
                a: self.a + o.b,
                b: self.b + o.b,
            }
        }
    }
}

fn main() {

    self::test1::hello();

    #[path = "one_file_module.rs"]
    mod module;
    //println!("Hello, world!");
    module::hello3();
    module::test2::hello_test();

    let a = opoverride::MyStruct{
        a: 1,
        b: 3,
    };

    let c = a.a;

    let c1 = String::from("123");
    let c2 = c1;

    println!("{}",c1);

    let b = opoverride::MyStruct{
        a: 2,
        b: 4,
    };
    let c = a + b;
    println!("c.a={}, c.b={}", c.a,c.b);
}
