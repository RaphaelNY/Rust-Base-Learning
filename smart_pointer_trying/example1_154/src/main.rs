use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),// Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // let a = Cons(5, &Nil); <- error. because &Nill creates a timelimited reference,its lifetime is
    // shorter than enum.
    {
        let a = Rc::new(Cons(5,
                            Rc::new(Cons(10,
                                Rc::new(Nil)))));
        // a.clone() is allowed. and it increases the reference count.
        println!("count after creating a = {}", Rc::strong_count(&a));
            {
                let b = Cons(3, Rc::clone(&a)); // let b = Cons(3,Box::new(a)); <- a borrowed here if Cons(i32, Box<List>)
                println!("count after creating b = {}", Rc::strong_count(&a));
                {
                    let c = Cons(4, Rc::clone(&a));
                    println!("count after creating c = {}", Rc::strong_count(&a));
                    println!("c = {:?}", c);
                } // c is dropped here
                println!("count after c goes out of scope = {}", Rc::strong_count(&a));
                println!("b = {:?}", b);
            } // b is dropped here
            println!("count after b goes out of scope = {}", Rc::strong_count(&a));
        println!("a = {:?}", a);
    } // a is dropped here
}