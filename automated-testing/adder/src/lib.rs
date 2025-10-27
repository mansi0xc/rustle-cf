pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_one_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn failed_test() {
        panic!("We shall intentionally make this fail.");
    }

    #[test]
    fn trial() {
        let yam = add(4, 5);
        assert_eq!(yam, 7);
    }

    #[test]
    fn iit_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 7 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}

/*
You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
To assert that an operation returns an Err variant, don’t use the question mark 
operator on the Result<T, E> value. Instead, use assert!(value.is_err()). */
