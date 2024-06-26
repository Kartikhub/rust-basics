use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numbers!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {}", secret_number);
    
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        if guess.trim() == String::from("quit") {
            println!("Exiting from the game");
            break;
        }

        println!("You guessed: {}", guess);

        // let guess: u32 = guess.trim().parse().expect("It is not an integer");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid integer. Try again.");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
