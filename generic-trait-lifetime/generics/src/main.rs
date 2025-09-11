/*
the function largest is generic over some type T. This function has one parameter named list, 
which is a slice of values of type T. The largest function will return a reference to a value 
of the same type T. */

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

/*
By declaring T as a generic type after impl, Rust can identify that the type in 
the angle brackets in Point is a generic type rather than a concrete type. */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/*
type Point<f32> will have a distance_from_origin method; other instances of Point<T> where 
T is not of type f32 will not have this method defined. */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("p.x = {}", p.x());

    // let wont_work = Point { x: 5, y: 4.0 };
}

/*
Using generic types won’t make your program run any slower than it would with concrete types.
Rust accomplishes this by performing monomorphization of the code using generics at compile time. 
Monomorphization is the process of turning generic code into specific code by filling in the 
concrete types that are used when compiled. 
*/