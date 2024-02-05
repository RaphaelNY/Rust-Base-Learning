/// # 11.1 test
/// - test:
///     - function
///     - see tested function as we thought.
/// - test function body
///     - testing function
///     - be tested function
///     - assert
/// ## test function
/// - testing function need use test attribute to be marked.
///     - attribute is a rust code
///     - add #\[test] to the function can change function to testing function.
/// ## how to run test function
/// - use: "cargo test" to run all test function in the project.
/// - rust will build a Test Runner to run all test function.
/// # 11.2 Assert
/// ## use assert to check the result
/// - ture is access.
/// - false is panic.
/// ## use assert_eq! || assert_ne!
/// - from std.
/// - as same as == and !=.
/// - assert to false will panic and print the message.
/// # 11.3 add custom message
/// - assert!: assert!(condition, message)
/// - assert_eq!: assert_eq!(left, right, message)
/// - assert_ne!: assert_ne!(left, right, message)
/// - message will give to format! to format the message, can use {}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn smaller(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "2 + 2 != 4");
    }

    #[test]
    fn assert_to_ne() {
        let result = add(2, 2);
        assert_ne!(result, 5, "2 + 2 == 5");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn go_to_panic() {
        panic!("this test should be failed.");
    }
}
