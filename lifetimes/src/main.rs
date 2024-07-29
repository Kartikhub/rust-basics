use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let result;

    let novel = String::from("Call me Abra. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let i = ImportantExcerpt {
        part: first_sentence
    };
    // Panic for print novel without lifetime
    // let i = ImportantExcerptWithoutLifetime {
    //     part: first_sentence
    // };

    println!("{}", novel);

    // function will at take the smaller of the lifetimes of string 1 and string 2 in this case it will take for string 2
    {
        let string2 = String::from("xyz");
        // let result = longest(string1.as_str(), string2);
        result = longest(string1.as_str(), string2.as_str());
        println!("result is : {}", result); 
        // result = longest_with_generic_and_lifetime(string1.as_str(), string2.as_str(), );
        // println!("result is : {}", result);                 
    }
    // Panic as string2 will be out of scope now
    // println!("result is : {}", result);
}

fn longest_with_generic_and_lifetime<'a, T>(
    x: &'a str, 
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display {
        println!("Announcement!, {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime is Struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Third lifetime elision
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

struct ImportantExcerptWithoutLifetime {
    part: str,
}


// Panic as the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
