pub fn run() {
    // for simple conditional logic use if statement
    let answer = false;
    if answer == true {
        println!("O yeah it's true");
    } else if answer == false {
        println!("o no its false");
    }

    // for complex comparison use match statements
    let age = 41;
    match age {
        1 => println!("O you are a toddler"),
        2 | 3 | 4 | 5 | 6 | 7 => println!("Growing really fast! hn"),
        8..=12 => println!("calm down boy"),
        13..=19 => println!("teeny tiny teen!"),
        20..=30 => println!("My man!"),
        n => {
            if n > 30 {
                println!("With great power comes great responsibility!");
            }
        } 
        // this is for default case if no pattern matches
        //_ => println!("You bulling me!"),
    }


    //you can do much more with pointers
        //you can destructure element which is being compared
            //this can be tuple, struct, enum or even pointers

    // you can put guards in match statements which do a specific condition check before executing
    // a case

    let name = ("mabroor","ahmad");

    match name {
        // this if acts as a guard 
       (x,_) if x == "mabroor" => println!("Oh hai mabroor"),
       _ => println!("I don't care who you are")
    }
}
