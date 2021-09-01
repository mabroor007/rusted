pub fn gives_ownership_of_string() -> String {
    String::from("ninja")
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

pub fn borrows_a_string_and_returns_size(s: &String) -> i32 {
    s.len() as i32
}

pub fn run() {
    let mut some_string = gives_ownership_of_string();

    // this scope owns some_string so it can modify the variable
    some_string.push_str(" is a cool guy.");

    // redeclare with same name results in shadowing
    let some_string = takes_ownership_of_string_then_gives_back(some_string);

    // this takes the ownership of a clone not original performance!
    takes_ownership_of_string(some_string.clone());

    takes_ownership_of_string(some_string);

    // throws error because ownership was given away and some_string is not in scope
    // some_string.push_str(" and he is an engineer");

    let mut new_string = String::from("Wickshot");

    let size = borrows_a_string_and_returns_size(&new_string); // this function borrowed new_string from this scope

    new_string.push_str("how");

    println!("size of '{}' is = {}", new_string, size);
}
