use std::io;

fn main() {
    println!("Enter input: ");
    let mut format = String::new();

    io::stdin().read_line(&mut format).expect("Failed to read line.");
    println!("Entered format : {format}");
    println!("{}", format[0]);
}
