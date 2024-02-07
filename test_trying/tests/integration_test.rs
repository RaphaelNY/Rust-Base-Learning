/// ## 11.10 group tests
/// create a new folder named tests, in the same path as src folder.
/// all file in tests folder will be crate.
/// ### run ordered group test
/// - cargo test function-name
/// - cargo test --test file-name   this can run all group test in ordered file.
/// ## group test for binary crate
/// - if project is binary crate, only have src/main.rs, haven't src/lib.rs:
///     - cannot cteate group test in tests folder
///     - cannot use main.rs
/// - only binary crate can show function to other crate to use.
/// - binary crate means independent run.

use test_trying;
mod common;

#[test]
fn it_add_two() {
    assert_eq!(4, test_trying::add_two(2));
}