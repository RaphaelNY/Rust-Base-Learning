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
/// ## 13.4 closures can visit the environment, can visit all varies in the environment where it's defined
/// - how to get the value from the environment?
///     - get the ownership: FnOnce
///     - get the mutable reference: FnMut
///     - get the immutable reference: Fn
/// - when the closure first be used, the type & Trait of the closure is inferred. FnOnce{FnMut{Fn}}
///     - all closures implement FnOnce
///     - if the closure didn't move the environment, it will implement FnMut
///     - if the closure didn't change the environment, it will implement Fn
/// - used move can ordered closures must get the ownership of the environment
/// ## 13.5 iterators
/// in rust, iterator is lazy, means it won't do anything until it's used.
/// - iterator trait && next method
///     // pub trait Iterator {
///     //    type Item;
///     //    fn next(&mut self) -> Option<Self::Item>;
///     //  // methods with default implementations elided
///     // }
/// - type Item and Self::Item are associated types, means they are defined in the trait, but the implementer will define the concrete type.
/// - iterator only ordered next method.
///     - next: once return a Option, covered in Some(Item) or None
///         - Some(Item): the next value
///         - None: the iterator is over
///     when iterator next out a item, it will move the iterator to the next position.
/// ## 13.6 used iterator
/// - map: receive a closure, return a new iterator, the new iterator will cover the value that the closure return.
///
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello")); // when closures first be used, the type of the closure is inferred
     // error: let n = example_closure(5);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let x :i32 = 4;
    let equal_to_x = |z| z == x; // the closure can visit the environment where it's defined
     // fn equal_to_x(z: i32) -> bool {z == x} // error: the function can't visit the environment where it's defined,means it can't catch x.
    let y :i32 = 4;
    assert!(equal_to_x(y));
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // 13.4
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // the closure can visit the environment where it's defined
    // println!("can't use x here: {:?}", x); // error: the closure get the ownership of the environment
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // 13.5 iterators
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // the iterator is lazy, means it won't do anything until it's used.

    for val in v1_iter { // iterator:v1_iter was moved here, so it can't be used again.
        println!("Got: {}", val);
    }
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // iterator trait && next method

}
