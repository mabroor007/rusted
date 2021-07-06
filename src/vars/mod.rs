// This is the rust way of declaring a global variable
// using this variable is a sin in rust
static mut JET_SPEED: i32 = 232234;

// This will not changed through out the program
pub const MY_AGE: u8 = 10;

pub fn log_static() {
    // unsafe keword is needed when you access of change the static variable
    unsafe {
        println!("{}", JET_SPEED);
    }
}

pub fn run() {
    let mut num = 0;
    {
        // you can freeze mut variables in lower scope so that they don't change
        let num = num;

        println!("inner scope num {}", num);

        // This creates an error
        // num += 1;
    }
    num += 1;
    println!("outer scope num {}", num);
}
