fn main() {
    //ENUMS AND PATTERN MATCHING

    // Enums
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can call the same function for both diff enum values.
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_2 = IpAddr_v2::V4(String::from("127.0.0.1"));
    let loopback_2 = IpAddr_v2::V6(String::from("::1"));

    let home_3 = IpAddr_v3::V4(127, 0, 0, 1);
    let loopback_3 = IpAddr_v3::V6(String::from("::1"));
    println!("{:?}", loopback_3);

    let m = Message::Write(String::from("HALLO"));
    m.call();

    // Match and enums
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// Enum Definition
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("Fn Route called");
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// We can put dat into each enum variant
enum IpAddr_v2 {
    V4(String),
    V6(String),
}

// We can define different types for IpAddress values
#[derive(Debug)]
enum IpAddr_v3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Call method is called");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
