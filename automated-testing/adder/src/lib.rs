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
}
