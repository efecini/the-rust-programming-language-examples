struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 20,
    };

    println!("Area of the rectangle is {}.", rect1.area());
    println!("Can Rec1 hold Rec2? {}", rect1.can_hold(&rect2));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
