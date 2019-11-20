fn main() {
    if_state();
}

//rust中if-else是一个表达式
//所有的分支需要返回相同的数据类型或不返回
fn if_state() {
    let age = 7u32;

    let k = if_express(age);

    println!("test if-else statement: {}", k)
}
fn if_express(age : u32) -> u32 {
    let kind = if age < 5 {
        1
    }else if age < 16 {
        2
    }else {
        3
    };

    kind
}
