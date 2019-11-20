fn main() {
    integer_data();
    float_data();
    bool_data();
    char_data();
    pointer_data();
    fix_array_data();
    tuple_data();
}

fn integer_data() {
    #![allow(unused_variables)]
    let i1 = 1i8;
    println!("size of i8 is {}", std::mem::size_of::<i8>());
    let i2 = 2i16;
    println!("size of i16 is {}", std::mem::size_of::<i16>());
    let i3 = 3i32;
    println!("size of i32 is {}", std::mem::size_of::<i32>());
    let i4 = 4i64;
    println!("size of i64 is {}", std::mem::size_of::<i64>());
    let i5 = 5i128;
    println!("size of i128 is {}", std::mem::size_of::<i128>());
    let i6 = 6isize;
    println!("size of isize is {}", std::mem::size_of::<isize>());

    let u1 = 1u8;
    println!("size of u8 is {}", std::mem::size_of::<u8>());
    let u2 = 2u16;
    println!("size of u16 is {}", std::mem::size_of::<u16>());
    let u3 = 3u32;
    println!("size of u32 is {}", std::mem::size_of::<u32>());
    let u4 = 4u64;
    println!("size of u64 is {}", std::mem::size_of::<u64>());
    let u5 = 5u128;
    println!("size of u128 is {}", std::mem::size_of::<u128>());
    let u6 = 6usize;
    println!("size of usize is {}",std::mem::size_of::<usize>())
}

fn float_data() {
    #![allow(unused_variables)]
    let f1 = 10.0f32;
    println!("size of f32 is {}", std::mem::size_of::<f32>());
    let f2 = 10.0f64;
    println!("size of f64 is {}", std::mem::size_of::<f64>());
}

fn bool_data() {
    #![allow(unused_variables)]
    let b = false;
    println!("size of bool is {}", std::mem::size_of::<bool>());
}

fn char_data() {
    #![allow(unused_variables)]
    let c = 'a';
    println!("size of char is {}", std::mem::size_of::<char>());
}

fn pointer_data() {
    #![allow(unused_variables)]
    let a = 10i8;
    let b = 10.0f32;
    let c = true;

    let ref pa = a;
    let ref pb = b;
    let ref pc = c;

    println!("size of &i8 is {}", std::mem::size_of::<&i8>());
    println!("size of &f32 is {}", std::mem::size_of::<&f32>());
    println!("size of &bool is {}", std::mem::size_of::<&bool>());
}

fn fix_array_data() {
    #![allow(unused_variables)]
    let a1 = [1,2,3,4,5];
    display_array(&a1);
    let a2:[i8;5] = [1,2,3,4,5]; //变量必须初始化后才能使用
    display_array(&a2);
    let a3 = [10i8;5];
    display_array(&a3);

    let ref s = a3[1..4];
    display_slice(s);
}

fn display_array(a : &[i8;5]) {
    let size =  a.len();
    let mut i = 0;
    print!("fixed array: [");
    while i < size {
        print!(" {} ", a[i]);
        i += 1;
    }
    println!("]");
}

fn display_slice(s : &[i8]) {
    let size =  s.len();
    let mut i = 0;
    print!("slice: [");
    while i < size {
        print!(" {} ", s[i]);
        i += 1;
    }
    println!("]");
}

fn tuple_data() {
    let t1 = (1,1.0,true);
    display_tuple(&t1);
    let t2 : (i8,f64,bool) = (10,10.0,false);
    display_tuple(&t2);
}
fn display_tuple(t : &(i8,f64,bool)) {
    println!("({},{},{})", t.0,t.1,t.2)
}
