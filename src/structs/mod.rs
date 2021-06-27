//  structs are object representations more like classes
//      but a lot more simpler as they name suggests
//      they just tell use what is the structure of the
//      object
#[derive(Debug)]
struct Field {
    val: i32,
}

// struct within struct
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    data: Field,
}

// thats how you implement method on structs
impl Person {
    // this function acts like a constructor
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
            data: Field { val: 0 },
        }
    }

    #[allow(dead_code)]
    pub fn bring_me_person() -> Person {
        Person {
            name: String::from("Cheeda talli"),
            age: 20,
            data: Field { val: 234 },
        }
    }

    pub fn inherit_person(self, child_name: &str) -> Person {
        // new structs can be created from previous structs
        //   props can be altered
        Person {
            name: child_name.to_string(),
            ..self
        }
    }

    // public method
    pub fn log(&self) {
        self.test();
        println!("{:?}", self);
    }

    // this is a private method
    fn test(&self) {
        println!("testing {}", self.name);
    }
}

// tuple struct
#[derive(Debug)]
pub struct PairOfWord(String, String);

impl PairOfWord {
    pub fn new(word1: &str, word2: &str) -> PairOfWord {
        PairOfWord(word1.to_string(), word2.to_string())
    }
    pub fn log(&self) {
        println!("{} paris with {}", self.0, self.1)
    }
}

// this is more like a singleton
//   useful for attatching method that belong
//   to a particular domain
pub struct Camera;

#[derive(Debug)]
pub struct Picture {
    pixels: [u8; 5],
}

impl Camera {
    pub fn take_picture() -> Picture {
        Picture { pixels: [0; 5] }
    }
}

// a function to test all above code
pub fn run() {
    let mabroor = Person::new("Mabroor Ahmad", 20);
    mabroor.log();
    let child = mabroor.inherit_person("Idk");
    PairOfWord::new("Effect", "Affect").log();
    let pic = Camera::take_picture();
    println!("{:?}", pic);

    // destructuring in rust
    // by redeclaring
    let Person {
        name: child_name, ..
    } = child;
    println!("child name is {}", child_name);

    // or by extracting .. is essential
    let Person { name, .. } = Person::new("Avatar", 21);
    println!("{}", name);

    // destructuring a tuple struct
    let PairOfWord(first, last) = PairOfWord("test".to_string(), "best".to_string());
    println!("first {} last {}", first, last);
}
