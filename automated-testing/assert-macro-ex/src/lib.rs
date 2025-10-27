#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn custom_greeting(name: &str) {
    format!("Hello {name}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_hold_func() {
        let larger = Rectangle {
            width: 9,
            height: 6,
        };

        let smaller = Rectangle {
            width: 4,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn rev_test_can_hold() {
        let larger = Rectangle {
            width: 12,
            height: 10,
        };

        let smaller = Rectangle {
            width: 7,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));

        // assert_eq!(smaller.can_hold(&larger), false); => works same
    }

    #[test]
    fn test_custom_greeting() {
        let result = custom_greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, output was {result}");
    }
}
