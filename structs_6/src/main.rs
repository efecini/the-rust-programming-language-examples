#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

fn main() {
    // Structs
    // Struct Example
    let user1 = User {
        active: true,
        username: String::from("efecini"),
        email: String::from("efecini@gmail.com"),
        sign_in_count: 1,
    };
    println!("User1's email is {}.", user1.email);

    // Mutable Struct Example

    let mut user2 = User {
        active: false,
        username: String::from("emiryorulmaz"),
        email: String::from("emiryorulmaz@gmail.com"),
        sign_in_count: 3,
    };
    println!("Before: User2's email is {}.", user2.email);
    user2.email = String::from("emiryorulmaz@ikmail.com");
    println!("After: User2's email is {}.", user2.email);

    let user3 = build_user(
        String::from("sehnazcini@gmail.com"),
        String::from("sehnazcini"),
    );
    println!("User3's email is {}.", user3.email);

    //You can use other structs and just add the fields that you want to change with ..user1
    let user4 = User {
        username: String::from("Ivan Divandelen"),
        ..user1
    };
    println!("User4: {:?}", user4);
    // println!("User1: {:?}", user1); // We can't use this one bc
    // Right now, User4 uses user1's email string which is moved to User4 now.
    // If it wasn't any string of User1's field that is moved into User4,
    // We could still use (and print) User1

    //Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    //there is no semicolon in the end which means that function returns the user object
    // We can use username&email since parameters have the same name with the struct fields.
    User {
        active: true,
        username,
        email,
        sign_in_count: 4,
    }
}
