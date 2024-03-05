use closures_trying::*;

/// # 13.0 closures
/// - Closures
/// - iterators
/// - improve the code of 12.
/// - other
/// ## 13.1 closures(1) - use closures to create a function
/// ### what's the closure?
/// - is a function that can capture the enclosing environment
/// - saved as a variable or a parameter
/// - can build in any scope and called in any other scope
/// - can catch the variables from the scope where it's defined
/// ## 13.2 closures(2) - type inference and annotation
/// ## 13.3
/// - create a struct whitch has closures and it working result
/// - cacher's limited
///     - only accept one closure
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); // when closures first be used, the type of the closure is inferred
     // error: let n = example_closure(5);
}
