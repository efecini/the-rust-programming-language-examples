fn main() {
    println!("Chapter 5: Ownership!");
    /*
    Ownership Rules:
    1. Each valur in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */
    {
        let s = "Hello";
        println!("{s}");
    }
    //println!("{}", s); //Won't work since s is out of scope

    //String Literal
    let s = "Hello";
    // s.pushstr(" World!") //Won't work

    let mut a = String::from("Hello");
    a.push_str(", world!"); //push_str() appends a literal to string
    println!("{a}");

    let x = 5;
    let y = x;

    println!("{x}");
    println!("{y}");

    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{s2}");

    // Ownership and Functions
    let s = String::from("Ownership");
    takes_ownership(s);
    //println!("{s}]"); //This won't work because now the function has the ownership of the value and memory is freed after takes_ownership function is done. And s is removed from the heap in the main.
    let x = 123;
    makes_copy(x);
    println("{x}"); //This is totally valid because integers are copied as an argument into the functions and main function still has the ownsership of the integer value.
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
