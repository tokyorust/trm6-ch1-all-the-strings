# Challenge #1 - Allow `&str` and `String`

For [Tokyo Rust Meetup's mini-hackathon on 2016-06-23](http://www.meetup.com/Tokyo-Rust-Meetup/events/231555496/).

## Setup

For this task, any Rust nightly should be fine.

## The challenge

It's simple! You have a `Person` struct whose `new` function only takes a `String`, but the cognitive overload of having to type `"...".to_owned()` or `String::from("...")` all the time when using it gets old real quick. Let's make the same function accept a `&str` as well.

Simply make `cargo test` pass without removing any tests and you're done! For convenience, `cargo run` already works and can be used for quick testing.

## (Optional bonus challenge)

Make `Person` displayable, so that it can be used with `println!()` without accessing the name directly. Your `main` should then look as follows:

```rust
fn main() {
    let you = Person::new("Challenger");
    println!("Hello {}!", you);
}
```
