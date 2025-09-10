fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    /*
    The or_insert method on Entry is defined to return a mutable reference to the value for 
    the corresponding Entry key if that key exists, and if not, it inserts the parameter as 
    the new value for this key and returns a mutable reference to the new value. */

    println!("{scores:?}");
}