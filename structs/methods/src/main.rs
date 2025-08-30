#[derive(Debug)]

struct Rectangle{
    width: u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32) -> Self{
        Self{
            width:size,
            height:size,
        }
    }
}
fn main() {
    println!("Hello, world!");

    let rec1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("rec1 is {:?}", rec1);
    println!("The area of the rectangle is {} square pixels.", rec1.area());
    println!("The width of the rectangle is {}", rec1.width);
    println!("The rectangle has a nonzero width: {}", rec1.width());

    println!("\nCan rect1 hold rect2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rec3));

    let sq = Rectangle::square(3);
    println!("\nThe square is {:?}", sq);
}
