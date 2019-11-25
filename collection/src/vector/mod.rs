use std::fmt::Display;

pub fn run() {
    use_vec();
}


fn list_vec<T:Display>(v: &Vec<T>) {
    print!("vector: [");
    for i in v {
        print!(" {} ", i)
    }
    println!("]")
}

fn use_vec() {
    //利用vec!宏构造
    let mut v = vec![1,2,3,4];
    list_vec(&v);

    //利用方法构造
    let mut v2:Vec<i32> = Vec::new();
    v2.push(21);
    v2.push(22);
    v2.push(23);
    v2.push(24);
    list_vec(&v2);

    //读取
    let t = v[0];
    println!("[vector]: v[0]={}", t);
    let r = v.get(1);
    match r {
        Some(v) => println!("[vector]: v[1]={}",v),
        None => println!("no such slot: 1"),
    }
    let r2 = v.get(100);
    match r2 {
        Some(v) => println!("[vector]: v[100]={}",v),
        None => println!("no such slot: 100"),
    }
    //修改
    v[1] = 1000;
    list_vec(&v);

    //删除
    v.pop();
    v.pop();
    list_vec(&v);
}
