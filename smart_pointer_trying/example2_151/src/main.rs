use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Cons(i32, List), <- this is not allowed,you can use Box, Rc, & to solve this
    Nil,
}