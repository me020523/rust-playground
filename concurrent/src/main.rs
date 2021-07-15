use crossbeam;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
}

#[test]
fn spawn_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(100))
        }
    });

    for _ in 1..5 {
        thread::sleep(Duration::from_millis(2000))
    }
}

#[test]
fn joined_thread() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handler.join().unwrap();
    println!("thread joined");
}

#[test]
fn move_thread() {
    let vec = vec![1, 2, 3, 4];

    //编译错误, vec使用borrow, 生命周期会有问题
    //let handler = thread::spawn(|| println!("here is a vector: {:?}", vec));

    let handler = thread::spawn(move || {
        println!("here is a vector: {:?}", vec);
    });

    handler.join().unwrap();
}

#[test]
fn thread_two_channel() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let s = String::from("hello, rust");

        println!("send msg: {}", s);
        tx.send(s).unwrap();
    });

    let v = rx.recv().unwrap();
    println!("recv msg: {}", v);
}

#[test]
fn thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(h);
    }

    for h in handlers {
        h.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap())
}

#[test]
fn crossbeam_find_max() {
    let v = &[1, 5, 3, 7, 10, 2, 0, 9];

    let max = find_max(v).unwrap();

    assert_eq!(10, max);
}

fn find_max(s: &[i32]) -> Option<i32> {
    if s.len() < 2 {
        return s.iter().cloned().max();
    }

    let mid = s.len() / 2;
    let (left, right) = s.split_at(mid);

    crossbeam::scope(|s| {
        let lt = s.spawn(|_| find_max(left));
        let rt = s.spawn(|_| find_max(right));

        let max_l = lt.join().unwrap()?;
        let max_r = rt.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}
