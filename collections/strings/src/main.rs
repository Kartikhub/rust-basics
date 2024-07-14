fn main() {

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }


    let mut s = String::from("foo");
    // push_str borrows the value of "s2" and not taking the ownership
    let s2 = "bar";
    s.push_str(s2);
    s.push('l');
    println!("{s} and s2 is {s2}");

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // note s3 has been moved here and can no longer be used
    // println!("{s3}");

    let s6 = String::from("tic");
    let s7 = String::from("tic");
    let s8 = String::from("tic");

    let s = format!("{s6}-{s7}-{s8}");
    println!("{s}");
}
