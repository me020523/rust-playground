use rand::distributions::Distribution as RDistribution;
use rand::distributions::{Alphanumeric, Standard};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal, NormalError};
fn main() {
    //simple_random();
    //range_random();
    //distribution_sample()
    //struct_random();
    //random_alpha();
    random_custom_alpha();
}

fn random_custom_alpha() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789)(*&^%$#@!~";
    const PASSWD_LEN: i32 = 8;
    let mut rng = thread_rng();

    let passwd: String = (0..PASSWD_LEN)
        .map(|_| {
            let idx = rng.gen_range((0..(CHARSET.len())));
            CHARSET[idx] as char
        })
        .collect();

    println!("{}", passwd)
}

fn random_alpha() {
    let rand_str: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    println!("{}", String::from_utf8(rand_str).unwrap());
}

fn simple_random() {
    let mut rng = thread_rng();

    let u: u8 = rng.gen();
    println!("u8 random: {}", u);

    let u: u16 = rng.gen();
    println!("u16 random: {}", u);

    let u: u32 = rng.gen();
    println!("u32 random: {}", u);
}

fn range_random() {
    let mut rng = thread_rng();
    println!("random integers {}", rng.gen_range(0..=10));
    println!("random floats {}", rng.gen_range(0.0..=10.0));

    println!("random integers {}", rng.gen_range(0..1000));
}

fn distribution_sample() {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);
    println!("{}", v)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: f32,
}

impl RDistribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y): (i32, f32) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn struct_random() {
    let mut rng = thread_rng();

    let rand_tuple = rng.gen::<(i32, f64, bool)>();
    let rand_point = rng.gen::<Point>();

    println!("tuple: {:?}", rand_tuple);
    println!("point: {:?}", rand_point);
}
