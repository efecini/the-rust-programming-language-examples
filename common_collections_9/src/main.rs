fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v_4 = vec![1, 2, 3];
    println!("{:?}", v_4);

    //Push to vector
    v_4.push(4);
    println!("{:?}", v_4);

    //Get elements
    let v_12 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v_12[2];
    println!("{third}");
    let thirdd = v_12.get(3);
    println!("{:?}", thirdd); //Wont work without debug_log because it returns an Option

    //Iterating in vectors
    let mut v_19 = vec![23, 234, 25];

    for i in &mut v_19 {
        *i += 5;
        println!("{i}");
    }

    // Strings
    let mut s = String::new();
    let data = "Some string";
    let s = data.to_string();
    println!("{s}");

    //Grow a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}"); // Prints toobar

    //You cant do indexing in strings

    //Slicing strings. You give bytes
    let hello = "Hello World";
    let s = &hello[0..4];
    println!("{s}");
    let s2 = &hello[0..1];
    println!("{s2}");

    //prints the bytes of H / 72 in ascii
    for b in s2.bytes() {
        println!("{b}");
    }

    //Hashmaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
