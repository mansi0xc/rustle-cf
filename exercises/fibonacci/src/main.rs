use std::io;

fn fibo(n: u32) -> u32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    fibo(n-1) + fibo (n-2)
}

fn fiboi(n: u32) {
    let mut a = 0;
    let mut b = 1;
    let mut i = 0;

    while i != n {
        println!("{a}");
        let t = a;
        a = a+b;
        b = t;
        i = i+1;
    }
}

fn main() {
    println!("Enter sequence number : ");
    let mut seq = String::new();
    io::stdin().read_line(&mut seq).expect("Failed to read line.");
    let seq:u32 = seq.trim().parse().expect("Please enter valid input - unsigned number");

    println!("Sequence required = {seq}");

    println!("Fibonacci number = {}", fibo(seq));

    fiboi(seq);
}
