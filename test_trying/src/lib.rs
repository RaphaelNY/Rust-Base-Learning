/// # TEST
/// ## 11.1 tests
/// - tests:
///     - function
///     - see tested function as we thought.
/// - tests function body
///     - testing function
///     - be tested function
///     - assert
/// ### tests function
/// - testing function need use tests attribute to be marked.
///     - attribute is a rust code
///     - add #\[tests] to the function can change function to testing function.
/// ### how to run tests function
/// - use: "cargo tests" to run all tests function in the project.
/// - rust will build a Test Runner to run all tests function.
/// ## 11.2 Assert
/// ### use assert to check the result
/// - ture is access.
/// - false is panic.
/// ### use assert_eq! || assert_ne!
/// - from std.
/// - as same as == and !=.
/// - assert to false will panic and print the message.
/// ## 11.3 add custom message
/// - assert!: assert!(condition, message)
/// - assert_eq!: assert_eq!(left, right, message)
/// - assert_ne!: assert_ne!(left, right, message)
/// - message will give to format! to format the message, can use {}
/// ## 11.4 use should_panic to deal that program should panic in our set situation
/// - tests will check the retun value is ture and check append panic in our set situation.
/// - should_panic's attribute
///     - function panic: True
///     - function did not panic: False
/// - you can add expected key to order the message that panic should have,like:
///     - \#\[should_panic(expected = "...")]
///  ## 11.5 use Result<T, E> to tests
/// - use **Result<T, E>** to tests the function that return Result.
///     - if return is Ok, tests is pass or if return is Err, tests is fail.
/// - don't used should_panic to tests the function that return Result.
/// ## 11.6 control the tests
/// - change the cargo tests motion to control the tests.-- add terminal varies.
/// - default:
///     - run all tests function.
///     - catch (but not show) the output.
/// - use:
///     - cargo tests --help (show --help can follow what command)
///     - cargo tests -- --help (show -- all command --help)
/// - more than one tests function cannot use a same situation, like environment, work-folder, environment path.
/// - --tests-threads
///     - cargo tests -- --tests-threads=1 (run tests function one by one)
/// - show-type function output
///     - default: if tests = ture,, if will catch all output to standard output.(for example, if tests ture, message in println! will not show)
/// ## 11.7 run tests by name
/// - use: tests name as car tests's varies.
///     - cargo tests test_name, can order one tests function to run.
///     - used apart of name to run more than one tests function(you can also use module name to run all tests function in the module)
/// ## 11.8 use command to ignore some tests, run others.
/// - add \#\[ignore] to the function to ignore the function.if we wanted to run this function, we can use cargo tests -- --ignored to run the function.
/// ## 11.9 tests orgnaization
/// - tests has group tests and single tests.
/// - single tests
///     - small,
///     - fast,once run a small module only.
///     - can tests private.
/// - group tests:
///     - in outside of lib,used it like other outside code.
///     - can only tests public.
///     - in one tests may used more than one function.
/// - \#\[cfg(tests)]
///     - only run cargo tests can run these code
/// - but, group tests needn't cfg attribute, it can in mane different folder-path
/// - cfg (configuration
///     - told rust follow can only include by ordered conf.
///     - only cargo tests will pack include helper function and \#\[tests] marked function.


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

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value nust be between 1 and 100, got {}.", value)
        }

        Guess{
            value
        }
    }

    pub fn new1(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value)
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value)
        }

        Guess{
            value
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
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

    // #[tests]
    // fn go_to_panic() {
    //     panic!("this tests should be failed.");
    // }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new1(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_return_10(8);
        assert_ne!(value, 5);
    }
}
