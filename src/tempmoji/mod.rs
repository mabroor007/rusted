use std::io::prelude::*;

const TYPE: &str = "⌨  ";
const OPT: &str = "➤ ";
const CHOICE: &str = "⚙️  ";

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    flush_output();
}

fn wait_for_interaction() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read the output");
}

fn flush_output() {
    std::io::stdout()
        .flush()
        .expect("failed to write output to stdout");
}

fn read_input(prompt: &str, ico: &str) -> i32 {
    let mut input = String::new();
    print!("{} {}: ", ico, prompt);
    flush_output();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read the output");
    input
        .trim()
        .parse()
        .expect("failed to parse number from input")
}

pub fn run() {
    loop {
        clear_screen();
        println!("[ Temp converter ]");
        println!("{} Choices ", CHOICE);
        println!("{} press 1 for Kelvin to Celsius", OPT);
        println!("{} press 2 for Celsius to Kelvin", OPT);
        println!("{} press 0 to quit", OPT);

        let choice = read_input("enter a choice", TYPE);

        let output = match choice {
            0 => {
                println!("Thanks for using the application!");
                break;
            }
            1 => {
                clear_screen();
                read_input("please enter temprature in kelvin", TYPE) as f64 - 273.15
            }
            2 => {
                clear_screen();
                read_input("please enter temprature in celsius", TYPE) as f64 + 273.15
            }
            _ => {
                clear_screen();
                println!("🤬 invalid choice!");
                wait_for_interaction();
                continue;
            }
        };

        println!("🌡️ temprature {}", output);
        wait_for_interaction();
    }
}
