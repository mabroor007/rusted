pub fn sample() {
    // improved readablity
    let billion = 1_000_000_000u128;
    println!(
        "billion and billions and billions are {}",
        billion * billion * billion
    );

    // some logic
    println!("true && false = {}", true && false);

    // some useful conversions
    println!("5 in binary is {:b}", 5);
    println!("25 in hex is {:x}", 15);
    println!("0xfff in decimal is {}", 0xf);
}
