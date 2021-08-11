use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

pub fn init() {
    println!("Guessing game! says welcome..");

    let number = rand::thread_rng().gen_range(1..101);

    loop {
        prompt("Please enter your number:");

        let input = read_input();

        let (input_number, err) = parse_int_from_input(&input);

        if err == true {
            println!("You entered an invalid number!");
            continue;
        }

        match input_number.cmp(&number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}

fn parse_int_from_input(input: &String) -> (i32, bool) {
    match input.trim().parse::<i32>() {
        Ok(num) => (num, false),
        Err(_) => (0, true),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to get the input");

    return input;
}

fn prompt(text: &str) {
    // input prompt
    print!("{}", text);

    // you need to trigger flush to display prompt
    io::stdout().flush().expect("failed to flush output");
}
