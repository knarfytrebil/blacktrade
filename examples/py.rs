use std::env;
fn main() {
    println!("{:?}", env::home_dir().unwrap().as_path());
}
