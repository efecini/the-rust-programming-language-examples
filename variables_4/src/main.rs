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
    println!("{}", b[6]);
    let c = [1; 4]; // [1,1,1,1]
    println!("{:?}", c);
}
