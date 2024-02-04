 /// # lifetime
 /// - in rust, any reference must have a lifetime.
 /// - lifetimes are a way of telling the compiler that a reference is valid for a certain scope.
 /// - in most way, lifetimes are similar to generics.
 /// - when referance's lifetime may be connected in different way, we had must mark out the lifetime.
 /// ## the purpose of lifetimes
 /// - avoid  to dangling references
 /// ## borrow checker
 /// ## lifetime annotations in function signatures
 /// - lifetime annotations don't change how long any of the references live.
 /// ## signal of lifetime
 /// - &i32        // a reference
 /// - &'a i32     // a reference with an explicit lifetime
 /// -'a mut i32   // a mutable reference with an explicit lifetime
 /// ## lifetime elision
 /// - signaled in function signature
 /// ## lifetime in struct
 /// ## rule of lifetime
 /// there are three rule, if code didn't meet the rule, the code will not be compiled.
 /// these rule is for "fn" and "impl" block.
 /// - first, every input reference gets its own lifetime parameter.
 /// - second, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
 /// - third, if there are multiple input lifetime parameters, but one of them is "&self" or "&mut self", the lifetime of self
 ///                                                                         is assigned to all output lifetime parameters.

 struct ImportantExcerpt<'a> {
     part: &'a str,
 }

fn main() {
    {
        let r;                                          // ---------+-- 'a
        {                                                     //          |
            let x = 5;                                   // --+-- 'b |
            r = &x;                                           //   |      |
        } // x is dropped here,so r is a dangling reference.  // --+      |
        println!("r: {}", r);                                 // ---------+
    }
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");

        let i = ImportantExcerpt {
            part: first_sentence
        };
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 // rust can't know return &str's lifetime from x or y,so we use "'a" to mark out the lifetime.
 // "'a" is the x_lifetime && y_lifetime's intersection in this code way.