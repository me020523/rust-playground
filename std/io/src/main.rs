fn main() {
    use std::io::Write;
    //println!("Hello, world!");
    std::fs::File::create("test.out")
        .unwrap()
        .write_all("hello,world".as_bytes())
        .unwrap()
}
