 /// # lifetime
 /// - in rust, any reference must have a lifetime.
 /// - lifetimes are a way of telling the compiler that a reference is valid for a certain scope.
 /// - in most way, lifetimes are similar to generics.
 /// - when referance's lifetime may be connected in different way, we had must mark out the lifetime.
 /// ## the purpose of lifetimes
 /// - avoid  to dangling references
 /// ## borrow checker

fn main() {
    {
        let r;                                          // ---------+-- 'a
        {                                                     //          |
            let x = 5;                                   // --+-- 'b |
            r = &x;                                           //   |      |
        } // x is dropped here,so r is a dangling reference.  // --+      |
        println!("r: {}", r);                                 // ---------+
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // rust can't know return &str's lifetime from x or y,so we use "'a" to mark out the lifetime.