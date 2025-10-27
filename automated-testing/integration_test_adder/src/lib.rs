pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(value: u32) -> u32 {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
