# About Unit and Integration Tests

## Unit Testing

"The purpose of unit tests is to test each unit of code in isolation from the rest of the code to 
quickly pinpoint where code is and isn’t working as expected. You’ll put unit tests in the src 
directory in each file with the code that they’re testing. The convention is to create a module 
named tests in each file to contain the test functions and to annotate the module with cfg(test)."
~ from the rust book.

Imp point to note - The #[cfg(test)] annotation on the tests module tells Rust to compile and run 
the test code only when you run cargo test, not when you run cargo build.

Also note - the attribute cfg stands for configuration

### Testing private functions :

```
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```

The internal_adder function is a private function - its not marked as pub. Tests are just Rust \
code, and the tests module is just another module. As discussed in “Paths for Referring to an Item 
in the Module Tree”, items in child modules can use the items in their ancestor modules. In this 
test, we bring all of the tests module’s parent’s items into scope with use super::*, and then the 
test can call internal_adder. 

## Integration Testing

"In Rust, integration tests are entirely external to your library. They use your library in the 
same way any other code would, which means they can only call functions that are part of your 
library’s public API. Their purpose is to test whether many parts of your library work together 
correctly. Units of code that work correctly on their own could have problems when integrated, so 
test coverage of the integrated code is important as well. To create integration tests, we first 
need a tests directory." ~ from the rust book

Note - We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test.

```cargo test --test <file name>``` => to run all the tests in a particular integration test file

