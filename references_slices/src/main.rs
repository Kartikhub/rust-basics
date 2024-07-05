fn main() {
    // Slice is basically reference to part of a String
    // let s = String::from("hello");

    // let slice = &s[0..2];
    // let slice = &s[..2];
    // let len = s.len();

    // let slice = &s[3..len];
    // let slice = &s[3..];

    // let len = s.len();

    // let slice = &s[0..len];
    // let slice = &s[..];

    // Scenario without slices - what if s changes after index
    // then the index will provide an incorrect value of s
    let mut s = String::from("Below code");
    let index = first_word(&s);
    println!("index is {index}");

    // Below code will throw error as s would be deallocated
    // once the dangle function scope ends.
    // let reference_to_nothing = dangle();
    // end

    // Below code will throw error as s1 and s2 are immutable
    // and used after declaring the mutable reference s3
    // let mut s = String::from("Hello");
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &mut s;

    // println!("s1 is {s1} and s2 is {s2}");
    // end
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
