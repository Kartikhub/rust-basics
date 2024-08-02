#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // custom messages can be added with assert
    #[test]
    fn exploration_with_message() {
        let result = add(2, 3);
        assert!(result == 5, "The provided value is incorrect, actual sum is {}", result);
    }

    // #[test] // add test attribute
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 10,
    //         height: 20
    //     };

    //     let smaller = Rectangle {
    //         width: 3,
    //         height: 5
    //     };
    //     assert!(larger.can_hold(smaller));
    // }

    // #[test]
    // fn another() {
    //     panic!("Fail the test");
    // }
}
