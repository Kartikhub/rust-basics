struct Point<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>)-> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }
    largest
}

fn main() {

    let p1 = Point {x: 5, y: 6.0};
    let p2  = Point {x: 12.09, y: 'c'};

    let p = p1.mixup(p2);

    println!("x is {} and y is {}", p.x, p.y);
    // let num_list = vec![12, 34, 67, 32, 12];
    // // panics as the all generic types cannot be ordered
    // let result = largest(&num_list);
    // println!("largest item is {}", result);
    // let char_list = vec!['y', 'q', 'a', 'z', 'c'];
    // let result = largest(char_list);
    // println!("largest item is {}", result);
}
