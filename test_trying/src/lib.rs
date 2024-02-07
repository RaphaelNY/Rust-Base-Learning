/// # TEST
/// ## 11.1 test
/// - test:
///     - function
///     - see tested function as we thought.
/// - test function body
///     - testing function
///     - be tested function
///     - assert
/// ### test function
/// - testing function need use test attribute to be marked.
///     - attribute is a rust code
///     - add #\[test] to the function can change function to testing function.
/// ### how to run test function
/// - use: "cargo test" to run all test function in the project.
/// - rust will build a Test Runner to run all test function.
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
/// - test will check the retun value is ture and check append panic in our set situation.
/// - should_panic's attribute
///     - function panic: True
///     - function did not panic: False
/// - you can add expected key to order the message that panic should have,like:
///     - \#\[should_panic(expected = "...")]
///  ## 11.5 use Result<T, E> to test
/// - use **Result<T, E>** to test the function that return Result.
///     - if return is Ok, test is pass or if return is Err, test is fail.
/// - don't used should_panic to test the function that return Result.
/// ## 11.6 control the test
/// - change the cargo test motion to control the test.-- add terminal varies.
/// - default:
///     - run all test function.
///     - catch (but not show) the output.
/// - use:
///     - cargo test --help (show --help can follow what command)
///     - cargo test -- --help (show -- all command --help)
/// - more than one test function cannot use a same situation, like environment, work-folder, environment path.
/// - --test-threads
///     - cargo test -- --test-threads=1 (run test function one by one)
/// - show-type function output
///     - default: if test = ture,, if will catch all output to standard output.(for example, if test ture, message in println! will not show)
/// ## 11.7 run test by name
/// - use: test name as car test's varies.
///     - cargo test test_name, can order one test function to run.
///     - used apart of name to run more than one test function(you can also use module name to run all test function in the module)
/// ## 11.8 use command to ignore some test, run others.
/// - add \#\[ignore] to the function to ignore the function.if we wanted to run this function, we can use cargo test -- --ignored to run the function.
/// ## 11.9 test orgnaization
/// - test has group test and single test.
/// - single test
///     - small,
///     - fast,once run a small module only.
///     - can test private.
/// - group test:
///     - in outside of lib,used it like other outside code.
///     - can only test public.
///     - in one test may used more than one function.
/// - \#\[cfg(test)]
///     - only run cargo test can run these code
/// - but, group test needn't cfg attribute, it can in mane different folder-path
/// - cfg (configuration
///     - told rust follow can only include by ordered conf.
///     - only cargo test will pack include helper function and \#\[test] marked function.


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
        assert_eq!(value, 5);
    }
}
