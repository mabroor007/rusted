// these are some basic loops

pub fn run() {
    let mut count = 0;
    // this is really cool
    //   you can label your loops and then use that label to break out of them
    //   this can be used to break out of outer loops if you have nested loops
    'counter: loop {
        if count > 10 {
            break 'counter;
        }
        println!("This would have ran infinite times, {} time", count);
        count += 1;
    }

    // let numbers = [234, 234, 10, 234];
    let mut vecs = vec![234, 634, 0345, 345];

    // iterating through vectors and arrays and mutating them
    for n in vecs.iter_mut() {
        // this will mutate the vector
        *n = 0;
        println!("{}", n);
    }

    // while loop
    let mut x = 0;
    while x < 1 {
        println!("{}", x);
        x += 1;
    }

    // loops can also return a value
    let mut bullets = 100;
    let remaining = loop {
        if bullets < 10 {
            break bullets;
        } else {
            bullets -= 1;
        }
    };

    println!("remaining bullets {}", remaining);
}
