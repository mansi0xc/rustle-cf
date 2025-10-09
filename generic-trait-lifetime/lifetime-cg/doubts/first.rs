fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}

// why is the error not about how we are assigning value to an immutabel vatiable and stright 
// about lifetime error?