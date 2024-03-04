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
///
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
