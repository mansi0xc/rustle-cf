#![allow(unused)]

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    /*incorrect snippet of code
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    */
}