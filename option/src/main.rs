use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

enum AgeLevel {
    Child,
    Teen,
    Youth,
    MiddleAge,
    Elderly
}

impl Display for AgeLevel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let result = match *self {
            AgeLevel::Child => {
                write!(f, "{}", "Child")
            },
            AgeLevel::Teen => {
                write!(f, "{}", "Teen")
            }
            AgeLevel::Youth => {
                write!(f, "{}", "Youth")
            }
            AgeLevel::MiddleAge => {
                write!(f, "{}", "MiddleAge")
            }
            AgeLevel::Elderly => {
                write!(f, "{}", "Elderly")
            }
        };
        return result;
    }
}

fn classify_by_age(age: u8) -> Option<AgeLevel> {
    match age {
        _ if age <= 8 => Some(AgeLevel::Child),
        _ if age > 8 && age < 16 => Some(AgeLevel::Teen),
        _ if age >=16 && age <= 35 => Some(AgeLevel::Youth),
        _ if age > 35 && age < 55 => Some(AgeLevel::MiddleAge),
        _ => Some(AgeLevel::Elderly),
    }
}

fn main() {
    let result = classify_by_age(32);
    match result {
        Some(level) => {
            println!("level is {}", level)
        }
        None => {
            println!("unknown age level")
        }
    }
}
