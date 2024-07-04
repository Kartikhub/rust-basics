fn main() {
// iteration of collection 
let _a = [1,2,3,4,5];

// for element in a {
//     println!("the value is : {element}");
// }
// Using Range and reverse the range

for number in (1..4).rev() {
    println!("numbers is {number}");
}


// end





// breaks and continue in nested loops with labels
// let mut count = 0;
// 'parent_loop: loop {
//     println!("count = {count}");
//     let mut remaining = 10;
//     loop {
//         println!("remaining = {remaining}");
//         if remaining == 9 {
//             break;
//         }
//         if count == 2 {
//             break 'parent_loop;
//         }
//         remaining -= 1;
//     }
//     count += 1;
// }
// println!("End count = {count}");
//  end

// loops with break and pass the outcome of the loop 
    // let mut i = 0;
    // let result = loop {
    //     i += 2;
    //     if(i > 10) {
    //         break i*i;
    //     }
    // };
    // println!("{}", result);
    // end
}
