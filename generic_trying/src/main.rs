/// # generic_trying
/// - generic used to solve the problem of code duplication
/// - generic worked like a model. It is a way to define a model that can be used in different places
/// - when IDE worked, some thing in it will be replaced by the type that we want to use
/// - for example: fn largest<T>(list: &[T]) -> T {}
/// ## type varies
/// - it always be short
/// - CamelCase
/// - T is type

struct Point<T, U> {
    x: T,
    y: U,
} impl<T, U> Point<T, U>{
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        } // return a Point with the same type of x and y, but the y is from the other Point
    }
} impl<I32, U> Point<I32, U>{
    fn x1(&self) -> &I32 {
        &self.x
    }
 // only <i32, U> type input has this function x1.
}// by this way, x and y can be the different type or the same type.


enum Option<T>{
    Some(T),
    None,
} // by this way, we can use the same type or different type in the enum

enum Result<T, E>{
    Ok(T),
    Err(E),
}

 // monomorphization: when IDE working, the generic will be replaced by the type that we want to use at first.
fn main() {
    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1, y: 4.0 };
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
} // this need trait of '>'