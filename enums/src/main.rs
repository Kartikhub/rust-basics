fn main() {
    let m = Message::Write(String::from("MJ"));
    m.call();

    // Explictly provide type otherwise -> panic
    let absent_num: Option<i32> = None;
    
    let i: Option<i32> =None;
    println!("value of i is {i:?}");

}

// enums and variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32,i32),
}

impl Message {
    fn call(&self) {
        println!("Message enum call");
    }
}
