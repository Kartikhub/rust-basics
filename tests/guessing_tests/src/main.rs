#[derive(Debug)]
struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("value should be greater than or equal to 1");
        } else if value > 100 {
            panic!("value should be less than or equal to 100");
        }

        Guess { value }
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // should panic will always panic even if the reason of panicking is different
    // #[should_panic]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}