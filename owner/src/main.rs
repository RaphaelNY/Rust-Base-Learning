// every x has a scope
// example
/*
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
}
