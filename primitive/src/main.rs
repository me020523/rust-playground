use std::convert::TryInto;

fn main() {
    println!("Hello, world!");
}

#[test]
fn slice_into_array() {
    let s1: [u8; 5] = [1, 2, 3, 4, 5];

    let convert = |s: &[u8]| -> [u8; 5] { s.try_into().expect("convert error") };

    let s2 = convert(&s1[..]);

    println!("{:#?}, {:#?}", s1, s2);

    assert!(true);
}

struct Empty;

impl Empty {
    fn display(&self) {
        println!("compile success")
    }
}

#[test]
fn empty_struct_call_method() {
    Empty.display()
}

#[test]
fn int_vec_sort() {
    //stable sort
    let mut vec = vec![1, 3, 2, 5, 4];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    //unstable sort
    let mut vec = vec![1, 3, 5, 2, 4];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn float_vec_sort() {
    let mut vec = vec![1.0, 3.0, 2.0, 5.0, 4.0];
    //vec.sort(); //compile error
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

#[test]
fn struct_vec_sort() {
    let mut vec = vec![
        Person::new("Bob".to_string(), 25),
        Person::new("Tom".to_string(), 30),
        Person::new("Jack".to_string(), 37),
    ];
    vec.sort();
    assert_eq!(
        vec,
        vec![
            Person::new("Bob".to_string(), 25),
            Person::new("Jack".to_string(), 37),
            Person::new("Tom".to_string(), 30),
        ]
    );

    vec.sort_by(|a, b| a.age.cmp(&b.age));
    assert_eq!(
        vec,
        vec![
            Person::new("Bob".to_string(), 25),
            Person::new("Tom".to_string(), 30),
            Person::new("Jack".to_string(), 37),
        ]
    )
}
