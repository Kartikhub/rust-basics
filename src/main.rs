
fn main() {

    //Understanding ownership
    let s1 = String::from("Hello universe");
    println!("Before - s1 - {}, pointer {:p}", s1, s1.as_ptr());
    let s2 = s1;
    // println!("After - s1 - {}", s1);
    println!("s2 - {} pointer - {:p}", s2, s2.as_ptr());
    // Understanding memory management
    // stack_fn();
    // heap_fn();
    // update_string();

    // // it will iterate from 0 to 9 -> [0,10) 
    // for i in 0..10 {
    //     println!("{}", i);
    // }

    // // iteration over a string
    // let str = String::from("My name is Anthony Gonsalvez");
    // println!("{}", get_first_word(str));


    // let is_cat = true;

    // if is_cat {
    //     print!("It is a cat");
    // } else if !is_cat {
    //     print!("It is not a cat");
    // }

    // let greeting = String::from("hello");
    // let welcome = "world";
    // println!("greeting {} and welcome {}", greeting, welcome);


    // It throws exception
    // let char1 = greeting.chars().nth(10);
    // print!("{}", char1.unwrap());


    // println!("Hello, world!");
    // let x: i8 = 1;
    // println!("x is {}", x);

    
    // let mut y: i8 = 100;
    // it will throw an error
    // for _i in 0..1000 {
    //     y = y + 100;
    // }
    // print!("y is {}", y);

}

fn _stack_fn() {
    let x = 1;
    let y = 2;

    println!("{} and {}" ,x, y);
}

fn _heap_fn() {
    let s1: String = String::from("Heap");
    let s2: String = String::from("memory");
    let combined = format!("{} {}", s1, s2);
    println!("{}", combined);
    println!("Capacity is {}, length is {} and pointer is {:p}", combined.capacity(), combined.len(), combined.as_ptr());
}

fn _update_string() {
    let mut str1: String = String::from("my string ");

    for _i in 0..1000 {
        str1.push_str(" abcdefghijklmnopqrstuvwxyz");
        println!("Capacity is {}, length is {} and pointer is {:p}", str1.capacity(), str1.len(), str1.as_ptr());
    }
}


fn _get_first_word(phrase: String) -> String {
    let mut ans = String::from("");
    for char in phrase.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return  ans;
}