// use std::time::Instant;

pub mod median_mode;
pub mod pig_latin;

pub mod text_interface;

fn main() {
    // let start = Instant::now();
    // let mut v = vec![1,2,3,4];
    // v.sort();
    // let ans = median_mode::median(&v);
    // println!("ans is {ans}");
    // let duration = start.elapsed();
    // println!("Time elapsed is {:?}", duration);

    // let mut v = vec![1,2,3,4,1,2,2];
    // let mode = median_mode::mode(&v);
    // println!("mode is {}", &mode);

    // let mut s = String::from("apple");
    // let ans = pig_latin::make_pl(&s);
    // println!("ans is {ans}");
    let mut v: Vec<String> = vec![String::from("Add Sally to Engineering"), String::from("Add Rahul to Engineering"), String::from("Add Amir to Sales"), String::from("Add Raj to Security")];
    let mut map = text_interface::show_people_in_dept(&mut v);

    let mut sorted_vec: Vec<_> = map.iter().collect();
    sorted_vec.sort_by_key(|a| a.0);
    println!("vec: {:?}", sorted_vec);

    for (key, value) in sorted_vec.iter() {
        println!("{}: {:?}", key, value);
    }
}
