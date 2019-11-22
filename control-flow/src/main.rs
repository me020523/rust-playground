fn main() {
    if_state();
    if_let();
    loop_state();
    while_state();
    while_let();
    for_state();
}

//rust中if-else是一个表达式
//所有的分支需要返回相同的数据类型或不返回
fn if_state() {
    let age = 7u32;

    let k = if_express(age);

    println!("test if-else statement: {}", k)
}
//rust中loop是一个表达式，通过break返回表达式的值
fn loop_state() {
    loop_express();
}

fn while_state() {
    let mut counter = 0;
    while counter <= 10000 {
        counter += 1;
        if counter > 1000 {
            break; //while-loop中break不返回值
        }
    }
    println!("while loop, counter: {}", counter)
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

fn if_let() {
    let x = Some(10);
    //x匹配枚举Some,并且参数值是5
    if let Some(5) = x {
        println!("if_let test true")
    }else{
        println!("if_let test false")
    }
}

//while中使用模式匹配
fn while_let() {
    let mut x = Some(0);
    //x匹配枚举Some, 且i接收x的参数值
    while let Some(i) = x {
        if i > 9 {
            println!("while let, i: {}", i);
            x = None
        }else {
            x = Some(i+1)
        }
    }
}

fn loop_express() {
    let mut counter = 0;

    let total = loop {
        counter += 1;
        if counter > 1000 {
            break counter;
        }
    };
    println!("loop counter: {}", total);
}

fn for_state() {
    let a = [1,2,3,4,5];
    print!("for interator 1: [");
    for i in a.iter() {
        print!(" {} ", i);
    }
    println!("]");

    let a = 1..10;
    print!("for iterator 2: [");
    for i in a {
        print!(" {} ", i);
    }
    println!("]");
}
