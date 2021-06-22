// With this directive in place rust will try
// its best to make this struct printable
// using {:?} in println! macro
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

// My-take
//   This is a bit wierd way of logging out structs in rust
//   I still don't know what this #[derive()] is it seems like
//   a decorator found in other languages.
