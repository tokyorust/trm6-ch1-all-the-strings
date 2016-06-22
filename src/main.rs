#![feature(plugin)]
#![plugin(clippy)]

use std::fmt;

struct Person {
    name: String,
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        Person {
            name: name.into(),
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    let you = Person::new("Challenger");
    println!("Hello {}!", you);
}

#[test]
fn should_accept_string_as_name() {
    Person::new("Joe".to_owned());
    Person::new(String::from("Joe"));
}

#[test]
fn should_accept_str_as_name() {
    Person::new("Joe");
}
