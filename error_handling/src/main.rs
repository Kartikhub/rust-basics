use std::fs::File;
use std::error::Error;

pub mod results;

// fn main() {
// main function can return result in this format
    fn main() -> Result<(), Box<dyn Error>>{
    results::read_username_from_file();
    Ok(())
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    // let greeting_file = File::open("hello.txt").unwrap();
    // println!("Hello, world!");
    // panic!("This is a panic!");
    // println!("This will not be printed");
    // let v = vec![1, 2, 3];
    // // RUST_BACKTRACE=1 cargo run  # to see the backtrace logs
    // v[99];
    
}
