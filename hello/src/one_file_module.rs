pub fn hello3() {
    println!("{}", "hello3")
}

pub mod test2 {
    pub fn hello_test() {
        let a = &String::from("hello");
        println!("{}", a)
    }
}
