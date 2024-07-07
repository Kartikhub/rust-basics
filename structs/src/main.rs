fn main() {
    let user1 = User {
        active: true,
        name: String::from("Anthony"),
        email: String::from("ant@hon.com"),
        sign_in_count: dbg!(12*40)
    };
    // Note that the entire instance must be mutable;
    //  Rust doesn’t allow us to mark only certain fields as mutable. 

    // let user2 = User {
    //     email: String::from("user2@hon.com"),
    //     ..user1
    // };
// error -  ^^^^^^^^^^ value borrowed here after move (As user 1 name is already assigned to user 2 name)
    // println!("user 1 name : {user1:#?}");
    dbg!(&user1);
    // let user3 = build_user("user3@hon.com", "Clope");
}

// tupple structs
struct Color(i32, i32,i32);
struct Point(i32, i32, i32);


#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, name: String) -> User {
    // Since  parameter names and the struct field names are exactly the same
    User {
        active: true,
        email,
        name,
        sign_in_count: 1
    }


//    Creating a user instance using boilerplate - email and name -twice
    // User {
    //     active: true,
    //     email: email,
    //     name: name,
    //     sign_in_count: 1
    // }
    // end
}