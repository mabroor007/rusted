mod vars;

fn main() {
    vars::log_static();
    vars::run();
    println!("{}", vars::MY_AGE);
}
