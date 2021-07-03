#[derive(Debug)]
enum Gun {
    Akm,
    Uzi,
}

#[derive(Debug)]
enum Scope {
    X8,
    X6,
    X4,
    X3,
    X2,
    Dot,
}

/// This module explain use keyword with enums
pub fn run() {
    // specified enum values are availabe in scope below
    use Gun::{Akm, Uzi};
    println!("power is {:?} speed is {:?}", Akm, Uzi);

    // This glob use makes all enum values availabe in
    // scope below
    use Scope::*;
    println!(
        "scopes in pubg are {:?} {:?} {:?} {:?} {:?} {:?}",
        Dot, X2, X3, X4, X6, X8
    );

    // its useful in match statements where you have
    //   compare many values in enums
}
