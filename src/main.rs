#![feature(plugin)]
#![plugin(clippy)]

struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Person {
        Person {
            name: name,
        }
    }
}

fn main() {
    let you = Person::new("Challenger".to_owned());
    println!("Hello {}!", you.name);
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
