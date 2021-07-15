fn main() {
    println!("Hello, world!");
}

fn same_lifetime<'i>(a: &'i String, b: &'i String) -> &'i String {
    b
}

fn different_lifetime<'i, 'j>(a: &'i String, b: &'j String) -> &'j String {
    b
}

#[test]
fn test_same_lifetime() {
    let a = String::from("Hello");

    {
        let b = String::from("World");
        println!("{}", same_lifetime(&a, &b))
    }
}

#[test]
fn test_different_lifetime() {
    let a = String::from("Hello");

    let b: &String;

    {
        let c = String::from("World");

        b = different_lifetime(&a, &c);
    }

    println!("{}", b)
}
