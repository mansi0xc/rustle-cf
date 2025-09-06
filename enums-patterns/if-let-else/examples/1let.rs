#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;

    /*
        let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    // This should work but rust will throw a warning that it must
    // handle all possibilities - and there’s a reason for that: 
    // you might not have exhaustive patterns, in which case the 
    // program will compile but then panic at runtime.  
    */

 // instead we can use if let to handle only one case
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}