// Always immutable, can't use mut, can be declred in any scope.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("{THREE_HOURS_IN_SECONDS}");
}
