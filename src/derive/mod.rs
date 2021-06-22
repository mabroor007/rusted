// This is a bit wierd way of logging out structs in rust
// I still don't know what this #[derive()] is it seems like
// a decorator found in other languages.
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
    pub fn log(&self) {
        println!("{:?}", self)
    }
}
