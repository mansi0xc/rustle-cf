### Controlling how tests are run

We have 2 options for command line - one for cargo test and another for the test binaries.

```cargo test --help``` => cargo test

```cargo test -- --help``` => test binaries

```$ cargo test -- --test-threads=1``` => to run tests one by one, single thread only
this prevents tests being run in parallel threads

```$ cargo test -- --show-output``` => shows function outputs

```$ cargo test <test/function name>``` => this runs a single, specified test

```$ cargo test <shared function name>``` => filters and runs all tests which have the given name/word

```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```
the given example demonstrates how to ignore specified tests by adding the [#ignore] attribute.

```$ cargo test -- --ignored``` => this runs all the ignored tests

