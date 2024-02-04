use trait_trying::{Summary, Tweet};
use trait_trying::NewsArticle;


/// # Trait
/// - A trait tells the Rust compiler about functionality a particular type has and can share with other types.
/// - We can use traits to define shared behavior in an abstract way.
/// - trait bounds: order the generic to achieve the some special function.
/// - trait like the interface in other language.but hase different.
/// ## set a trait
/// - put the signature of the method that we want to share in the trait.
/// - use the keyword: trait.
/// - it can be more than one method in a trait
/// - if we do this trait,we need to did all method in this trait orderd, whitch trait was signed without init.
/// ## achieve trait in types
/// - work like achieve method in type
/// - different:
///     - called by: inpl Xxxx for Yyyy{}; Xxxx is the trait name, Yyyy is the type name.
/// ## achieve trait bounds
/// - can achieve some trait in some type need these:
///     - this tpye or this trait was signed in downland crate.
/// - we can't achieve the outside trait in the outside type.
/// ## init trait achieve

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

 // use trait bounds to fix
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
     let mut largest = list[0].clone();
     for item in list.iter() {
         if item > &largest { // error: the trait `PartialOrd` is not implemented for `T`, '>'this trait is achieved in std::cmp::PartialOrd
             largest = item.clone();
         }
     }
 } // for type that didnot achieve Copy trait.
fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T {
     let mut largest = list[0];
     for &item in list.iter() {
         if item > largest {
             largest = item;
         }
     }
     largest
 } // for type that achieved Copy trait.