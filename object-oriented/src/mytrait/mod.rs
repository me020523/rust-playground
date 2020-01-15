pub trait Human {
    fn think();
}

pub trait Programmer {
    fn program();
}

pub trait Writer {
    fn write();
}

pub struct Man {
    name: String,
    age : u8,
}

impl Man {
   fn new(age: u8, name: String) -> Man {
        Man {
            name,
            age,
        }
    }
    fn show(&self) {
        println!("my name is {}, {} years old", self.name, self.age)
    }
}
impl Human for Man {
    fn think() {
        println!("man is thinking")
    }
}

impl Programmer for Man {
    fn program() {
        println!("man is programming")
    }
}

impl Writer for Man {
    fn write() {
        println!("man is writting")
    }
}

struct ITCompany {
    worker: u32,
}

impl ITCompany {
    fn hire(&mut self, _w: impl Writer + Programmer) {
        self.worker += 1
    }
}

struct WriterComany {
    worker: u32,
}

impl WriterComany {
    fn hire(&mut self, _w: impl Writer) {
        self.worker += 1
    }
}

pub fn run() {
    let mut it = ITCompany{
        worker: 0,
    };
    let mut wt = WriterComany{
        worker: 0,
    };

    let bob = Man::new(25, String::from("bob"));
    bob.show();
    bob.show();

    let sam = Man::new(27, String::from("sam"));
    sam.show();
    sam.show();

    it.hire(bob);
    wt.hire(sam);

    println!("it company has workers: {}", it.worker);
    println!("writer company has workers: {}", wt.worker);
}
