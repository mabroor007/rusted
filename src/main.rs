mod derive;

fn main() {
    let mabroor = derive::Person::new("Mabroor Ahmad", 20);
    mabroor.log()
}
