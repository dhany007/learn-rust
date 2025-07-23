#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if value < 1 || value > 100{
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess{value}
    }
}

pub fn add(x: i32, y: i32) -> i32{
    x + y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle{
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle{
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    // assert equal
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // should panic
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100_should_panic() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 0.")]
    fn lower_than_1_should_panic() {
        Guess::new(0);
    }

    // using Result<T, E>
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(3,4);
        if result == 7 {
            Ok(())
        } else {
            Err(String::from("three plus four does not equal seven"))
        }
    }


    #[ignore]
    fn ignore_test() {
        // code that takes an hour to run
    }
}

// Running Tests in Consecutively
// cargo test -- --test-threads=1

// Showing Function Output
// cargo test -- --show-output

// Running Single Tests
// cargo test greater_than_100_should_panic

// Filtering to Run Multiple Tests
// cargo test add => This command ran all tests with add in the name

// Ignoring Some Tests Unless Specifically Requested
// cargo test -- --ignored => using #[ignore] in mod tests