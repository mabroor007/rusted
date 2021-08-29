pub fn gives_ownership_of_string() -> String {
    String::from("ninja")
}

pub fn run() {
    let mut some_string = gives_ownership_of_string();
    some_string.push_str(" is a cool guy.");

    // redeclare with same name results in shadowing
    let some_string = takes_ownership_of_string_then_gives_back(some_string);

    takes_ownership_of_string(some_string);

    // throws error because ownership was given away
    // some_string.push_str(" and he is an engineer");
}

pub fn takes_ownership_of_string(s: String) {
    println!("i took the ownership of '{}' and i won't give it back", s);
}

pub fn takes_ownership_of_string_then_gives_back(s: String) -> String {
    println!(
        "i took the ownership of '{}' and i will give it back now",
        s
    );
    s
}
