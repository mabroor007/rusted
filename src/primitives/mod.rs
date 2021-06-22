pub fn primitive_types() {
    let truth: bool = true;
    let decimal: f64 = 234.23; // defaults to f64
    let number: i128 = 234234234234234; // defaults to i32

    // there are some more types like
    // usize and isize
    //   these depend on underlying architecture

    println!(
        "bool = {boolean}, float = {float}, integer = {integer}",
        boolean = truth,
        float = decimal,
        integer = number
    );

    // this causes an error we can not assign again
    //number = 234;

    // but this does not cause an error
    let number: i32 = 349345;
    println!("shadowing variable {}", number);
    // reason is shadowing in rust
    // rust allows new variables to shadow
    // previous ones meaning after redeclaring
    // only new variable will be accessible in
    // the scope that below.
    // --- My-take
    // I am not a fan of this approach i don't
    // find much use of this right now but i'll
    // see how it goes.

    // some useful tricks
    let bin = 0b01101;
    println!("binary = {}", bin);

    let byte_litteral = b'A';
    println!("byte_litteral = {}", byte_litteral);
}
