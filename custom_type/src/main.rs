fn main() {
    std_struct();
    tuple_struct();
    std_enum();
    enum_with_data();
}
struct User {
    name: String,
    age: u32,
}
struct UserAttr(i32,i32, String);

fn std_struct() {
    let u1 = User {
        name: String::from("tom"),
        age: 16,
    };

    let u2 = User {
        name: String::from("lily"),
        ..u1
    };

    println!("u1, name:{}, age:{}", u1.name, u1.age);
    println!("u2, name:{}, age:{}", u2.name, u2.age);

    let u3 = local_assign(String::from("bob"),21);
    println!("u3, name:{}, age:{}", u3.name, u3.age);
}
fn local_assign(name: String, age: u32) -> User {
    User {
        name,
        age,
    }
}

fn tuple_struct() {
    let u1 = UserAttr(11,123, String::from("fat"));
    println!("u1_attr, {}, {}, {}", u1.0,u1.1, u1.2)
}

fn std_enum() {
    enum Kind {
        Fat,
        Skinny
    }
    let k = Kind::Fat;
}
fn enum_with_data() {
    #![allow(unused_variables)]
    enum Kind {
        Fat(String),
        Skinny(i32),
        Normal{weight:i32},
    }
    let k1 = Kind::Fat(String::from("fat"));
    let k2 = Kind::Skinny(36);
    let k3 = Kind::Normal{weight:26};
    match k3 {
        Kind::Fat(_) => {
            println!("i am fat")
        },
        Kind::Skinny(_) => {
            println!("i am skinny")
        },
        Kind::Normal{weight:w} => {
            println!("My weight is {}", w);
        }
    }
}
