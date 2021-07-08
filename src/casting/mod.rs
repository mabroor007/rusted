// so there is no implicit conversion in rust
//   you have to explicitly say using as keword
pub fn cast() {
    println!("integer 1000 to float {}", 1000 as f64);
    println!("float 1000.000 to integer {}", 1000.000 as i32);
    println!("float 2342.22 to integer {}", 2342.22 as i32);
    println!("integer 65 to char {}", 65 as char);
    println!("max int val {}", i8::MAX);

    // this is a bit strange
    //   it will not result in +65
    println!("signed integer -65 to unsigned integer {}", -10i8 as u64);
    // this creates an error because u8 can not accomodate such a large value
    //println!("32bit integer to an 8bit integer {}", 2342 as u8);
}
