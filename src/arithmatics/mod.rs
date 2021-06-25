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

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // shift bits to right
    println!("00000001 << 5 is {:b}", 1u8 << 6);
    // shift bits to left
    println!("00000001 >> 1 is {:b}", 1u8 >> 2);
}
