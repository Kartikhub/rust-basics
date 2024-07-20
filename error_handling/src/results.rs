use std::fs::{self, File};
use std::io::{self, Read};

pub fn read_username_from_file() -> Result<String, io::Error> {
    // let username_from_file_result = File::open("myfile.txt");

    // let mut username_file = match username_from_file_result {
    //     Ok(file) => file,
    //     Err(e)  => return Err(e),
    // };

    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Shorter way to write the same code
    // let mut username_file  =File::open("myfile.txt")?;
    // let mut username =String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // Even shorter way
    // let mut username = String::new();
    // File::open("myfile.txt")?.read_to_string(username)?;
    // Ok(username);

    // Shortest way
    fs::read_to_string("myfile.txt")
}