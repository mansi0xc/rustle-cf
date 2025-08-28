fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

    // let mut x = 3;
    // let y = x;

    // x = 5;
    // println!("x = {x}, y = {y}");

    let mut s2 = s;
    println!("{s2}");
    s2.push_str(" hiyaa");
    //println!("{s}"); 
    println!("{s2}");

    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

}
