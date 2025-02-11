#[allow(unused_variables)]

// Always immutable, can't use mut, can be declred in any scope.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 3;
    println!("The value of x is {x}");
    x = 2;
    println!("The value of x is {x}");
    println!("{THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let y = 5;
    println!("Y outside_1 is {}", y);
    let y = y + 1;
    println!("Y outside_2 is {}", y);
    {
        let y = y * 3;
        println!("Y in brackets_1 is {}", y);
    }
    println!("Y outside_3 is {}", y);

    // With shadowing, after the assignment, we have the same named variable as an immutable variable again.
    // We can change the value type with the same variable name.

    // You can't do the thing at the bottom without shadowing.(doing let 2 times)
    let spaces = "   "; // String
    let spaces = spaces.len(); // Int

    //Data Types
    let guess: u32 = "42".parse().expect("Not a number");

    let x: i8 = 127;

    let y = 2.4; // f64
    let yy = 3.0; // f32

    //Basic Ops
    let sum = 5 + 10;
    println!("Sum:	{sum}");
    let diff = 5 - 10;
    println!("Difference:	{diff}");
    let mult = 5 * 10;
    println!("Multiplication:	{mult}");
    let div = 5 / 3;
    println!("Divison:	{div}");
    let rema = 47 % 5;
    println!("Remainder:	{rema}");

    //Bool
    let t = true;
    let f: bool = false;
    println!("{f}");

    //Tuple
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    println!("{}", tup1.0);

    let (x, y, z) = tup2;
    println!("Middle value is {}", y);

    //Array
    let a = [1, 2, 3, 4, 6];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", b[2]);
    // println!("{}", b[6]); //index out of bonds
    let c = [1; 4]; // [1,1,1,1]
    println!("{:?}", c);
    another_function(43, 'p');

    //Expressions
    //This is an expression.
    let y = {
        let x = 3;
        x + 1
    };
    println!("Y is {}", y);
    println!("Five func returns: {}", five());

    let number = 3;
    // Conditional
    if number < 5 {
        println!("Condition was true.")
    } else {
        println!("Condition was false.")
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 8 };
    println!("Number is {number}");
    /*
    // Loops forever
    loop {
        println!("again and again");
    }
    */

    // while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("The value is {element}");
    }

    // Countdown with for loop+rev
    {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!!")
    }
}

fn another_function(x: i32, c: char) {
    println!("Printed from another function. Int is {x} and char is {c}");
}

fn five() -> i32 {
    5
}
