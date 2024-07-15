// use std::time::Instant;

pub mod median_mode;
pub mod pig_latin;

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

    let mut s = String::from("apple");
    let ans = pig_latin::make_pl(&s);
    println!("ans is {ans}");
}
