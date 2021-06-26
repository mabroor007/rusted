fn bring_me_tuple() -> (&'static str, i32) {
    ("a number", 20)
}

pub fn run() {
    let tuple = bring_me_tuple();
    println!("{:?}", tuple);

    // Destructuring tuple
    let (name, number) = bring_me_tuple();
    println!("{} = {}", name, number);
    // i like this approach its like destructuring in typescript/javacript

    // some limitations are that for now tuple larger than 12 elements
    // can not be printed automatically
}
