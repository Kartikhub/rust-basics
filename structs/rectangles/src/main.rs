fn main() {

    // Method in rust is defined within the context of a struct, enum or trait
    // and their first parameter is always self
    let mut rect1 = Rectangle {
        width: 50, height: 30
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(3);
    println!("square {square1:#?}");
    // println!("Can rect1 hold rect2? {}", rect1.can_hold_area(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold_area(&rect3));
    
    // println!("area is {}", rect1.area());
    // println!("square {:#?}", rect1.create_square_using_width(2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn create_square_using_width(&mut self, extended_width: u32) -> &Self {
        self.height = self.width + extended_width;
        self
    }

    fn can_hold_area(&self, other_rect: &Rectangle) ->bool {
        if self.area() >= other_rect.area() {
            return true;
        }
        false
    }
    // Associated functions - without self as parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

