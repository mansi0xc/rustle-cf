use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}

// we’re only allowed to use the ? operator in a function that returns Result, Option, 
// or another type that implements FromResidual.