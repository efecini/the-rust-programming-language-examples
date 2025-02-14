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
    //println!("{s}]"); //This won't work because now the function scope has the ownership of the value and memory is freed after takes_ownership function is done. And s is removed from the heap in the main.
    let x = 123;
    makes_copy(x);
    println!("{x}"); //This is totally valid because integers are copied as an argument into the functions and main function still has the ownsership of the integer value.

    //Here x goes out of the scope and s since s's value was moved.

    //There must be a better way to do this thing below. It is too much ceremony and lots of work.
    //What if we want to let a function use the value but not take ownership.
    let s1 = String::from("Hello");
    let (s1, len) = calculate_length(s1);

    //REFERENCE & BORROWING
    let s1 = String::from("Hello");
    let len2 = calculate_length_2(&s1);

    println!("The length of {s1} is {len2}");

    //Mutable Reference

    let mut s_56 = String::from("Hello");
    change(&mut s_56); // !!!!! DONT FORGET TO PUT MUT
    println!("{s_56}");

    // Rule: If you have a mutable reference to a value, you can have no other references to that value.
    let mut s_62 = String::from("Hello 62");
    let r1 = &mut s_62;
    println!("{r1}");
    //let r2 = &mut s_62; //This will crash.

    // Combining mutable and immutable references
    let s = String::from("Hello");
    let r1 = &s; //fine
    let r2 = &s; //fine bc this is also an immutable reference.
                 // let r3 = &mut s; // Big problem: Cannot borrow 's' as mutable since it is also borrowed as an immutable ref.
                 // println!("{r1},{r3},{r3},"); Won't work

    // Slices
    // A string slice is a reference to part of a String.

    let s_77 = String::from("hello world");
    let hello = &s_77[0..5];
    let world = &s_77[6..11];
    println!("{}, {}, {}", s_77, hello, world);

    //Example about string slices
    let mut s_83 = String::from("Hello world");
    let word = first_word(&s_83); // Immutable reference
                                  // s_83.clear(); //Error! Since we can't
    println!("The first word is {word}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here some goes out of the scope so "drop" is called.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} //here an integer goes out of scope. Nothing special happens.

fn calculate_length(s1: String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world !!!")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //If we find the first blank char, return the prior slice
        }
    }
    &s[..] //That means we couldn't find any blank so return the whole string
}
