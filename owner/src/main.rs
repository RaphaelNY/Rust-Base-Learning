/* every x has a scope
   example
    fn main() {
         // s unavailable
        let s :&str = "hello"; // s available
                               // can do something to s
    } // s's scope end
*/
fn main() {
    let s  = String::from("hello");
    let mut s:String = "hello".parse().unwrap();
    println!("{}",s);
    s = "Hello".parse().unwrap(); // way 1 to change the s
     // drop(s); if release s ahead of next used, it will be error
    s.push_str(" world!");
    println!("{}",s);
     /* str && String
        str cannot be changed ,because str was locked when s:str was built.
        String can be changed. because when s:String will be build, System will give a space
            for String in heap to make String's changeability.
            (String will call some space,and release it in some way)
     */
     // Move
    let x = 5;
    let y = x; // because i32 in a known value, this two 5 was pushed to stack
    let s1 = String::from("hello");
    let s2 = s1; // this has a different exchange way to last one
    println!("{} {}",y,s2);
     /*
        String has three part: a pointer
                             : length(how many Bs of save the thing in String needed is)
                             : capacity(String called the total Bs from System)
        what in String(such as "hello") was saved in heap
        when we used: s2 = s1;
        - in stack, pointer, len,capacity was copied
        - what pointer point path was didn't changed
            - it means,the thing in s1 wasn't copied
        when s1 and s2 leaved scope,they will try to release the same memory, which will cause **the double free bug**.
        to avoid this question,rust will make s1 lose efficacy,then when s1leaved scope, rust needn't free any memory.
     */

     // move || Clone
     /* shallow copy and deep copy
        shallow copy: copy the pointer, len, capacity
        deep copy: copy the pointer, len, capacity, and the thing in pointer point path
        rust use move to shallow copy
        if we want to deep copy, we can use clone
        eig. you can used clone to copy a String,which can copied data in heap
     */
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("example about clone: ");
    println!("s1 = {}, s2 = {}", s1, s2);
    drop(s1);
    drop(s2);
     /* but some varies only in stack, needn't be clone,of all these,shallow or deep seems the same
        can copy eig. i32, bool, char, f64, u32, etc.
        eig. if something or a part of it has achieved Copy trait, rust wouldn't allowed to achieve copy trait again
        can't copy eig. String, Vec<T>, etc.
        if all varies in a tuple is can be copy, the tuple can be copy
     */
    ////////////////////////////////////////////////////////////////////////////////////////////////
     // own power and function
     /*
        when a function was called, all of its parameters will be moved or copied
        eig. fn main() {
                let s = String::from("hello");
                takes_ownership(s);
                let x = 5;
                makes_copy(x);
            }
            fn takes_ownership(some_string: String) { // some_string come into scope
                println!("{}", some_string);
            } // some_string goes out of scope and `drop` is called. The backing memory is freed.
            fn makes_copy(some_integer: i32) { // some_integer comes into scope
                println!("{}", some_integer);
            } // some_integer goes out of scope. Nothing special happens.
     */
    let s = String::from("hello");
    takes_ownership(s.clone()); // takes_ownership(s); error, s has lost efficacy
    let x = 5;
    makes_copy(x);
    println!("s is {}",s); // error, because s has lost efficacy
    println!("x is {}",x); // right, because x is i32, which has achieved Copy trait
}

fn takes_ownership(some_string: String) { // some_string come into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}