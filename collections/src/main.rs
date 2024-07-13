fn main() {
    // adding type annotation here as not storing any value
    let mut v: Vec<i32> = Vec::new();
    // example of without adding type annotation
    let v1 = vec![1, 2, 3];
    v.push(5);
    v.push(7);

    let third: &i32 = &v1[2];
    println!("third value is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("value is {third}"),
        None => println!("there is no third element"),
    }

    for i in &v1 {
        println!("element is {i}");
    }

    for i in &mut v {
        // Usage of * to dereference operator
        *i += 50;
    }

    let row = vec![
        SpreadSheetCell::Int(1),
        SpreadSheetCell::Text(String::from("Language")),
        SpreadSheetCell::Float(23.65),
    ];
}

// Using enum to store multiple types in vectors

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
