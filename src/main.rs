
fn main() {

    let x= 6;
    let y = if x < 5 { x+5 } else {x-5};
    println!("{}", y);
    // let y = {
    //     let x = 1;
    //     x + 1
    // };
    // println!("{}", y);

    // let circle: Shape = Shape::Circle((5.0));
    // let square: Shape = Shape::Square((2.0));
    // let rectangle: Shape = Shape::Rectangle((2.0), (5.0));
    // print!("Area of circle is {} and for square is {} and for rectangle is {}", calculate_area(circle), calculate_area(square), calculate_area(rectangle));

    //Pattern matching with enums



    // ////# Basic -struct
    // let user: User = User {
    //     name: String::from("Kalki"),
    //     age: 1,
    //     id: 123456
    // };
    // ////# Basic - without implementation
    // println!("User name is {} and age is {}", user.name, user.age);

    // ////# With Implementation
    // user.get_name_and_age();
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}

fn calculate_area(shape: Shape) -> f64 {
    ////# Pattern matching 
    match shape {
        Shape::Circle(radius) => 3.14* radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side
    }
}

impl User {
    fn get_name_and_age(&self) {
        println!("from impl - User name is {} and age is {}", self.name, self.age)
    }
}

struct User {
    name: String,
    age: u32,
    id: u64
}


    // let mut s1 = String::from("my string");
    // let s2 = &mut s1;
    // // let s3 = &s1;
    // s2.push_str(" added");
    // println!("s1 - {}", s1);
    // println!("s2 - {}", s2);
    // println!("s3 - {}", s3);

    // let mut og_string = String::from("Hello");
    // ////# Passing variable - borrowing - update og_string to mutable for this particular case - Mutable references
    // _borrow_string_without_manipulation(&og_string);
    // _borrow_update_string(&mut og_string);
    // println!("{}", og_string);
    // ////# Passing variable - borrowing -- without manipulating the string
    
    // _borrow_string_without_manipulation(&og_string);
    // _borrow_string_without_manipulation(&og_string);
    

    ////# Passing variable without borrowing
    // takes_ownership(og_string);
    // println!("{}", og_string); // this would cause a compile error because ownership has been moved.
// }

fn _borrow_update_string(borrowed_string: &mut String) {
    borrowed_string.push_str(" added string");
    // println!("Borrowed and updated {}", borrowed_string.push_str(" added string")); // Throws error in this line 
}

fn _borrow_string_without_manipulation(borrowed_string: &String) {
    println!("Borrowed {}", borrowed_string);
}

fn _takes_ownership(new_string: String) {
    println!("{}", new_string);
}
    ////#  Understanding ownership basics
    // let s1 = String::from("Hello universe");
    // println!("Before - s1 - {}, pointer {:p}", s1, s1.as_ptr());
    // let s2 = s1;
    // println!("After - s1 - {}", s1);
    // println!("s2 - {} pointer - {:p}", s2, s2.as_ptr());
    
    
    ////# Understanding memory management
    // stack_fn();
    // heap_fn();
    // update_string();

    ////# it will iterate from 0 to 9 -> [0,10) 
    // for i in 0..10 {
    //     println!("{}", i);
    // }

    ////# iteration over a string
    // let str = String::from("My name is Anthony Gonsalvez");
    // println!("{}", get_first_word(str));


    ////# basics of rust - primitive types and conditionals and loops
    // let is_cat = true;
    // if is_cat {
    //     print!("It is a cat");
    // } else if !is_cat {
    //     print!("It is not a cat");
    // }

    // let greeting = String::from("hello");
    // let welcome = "world";
    // println!("greeting {} and welcome {}", greeting, welcome);


    // It would cause compile error
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

// }

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