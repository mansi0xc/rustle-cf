enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, world!");
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("Value of coin is: {} cents", value);
    // let rando :u32 = -32;
    // let tup = (500, 6.4, 1);
}
