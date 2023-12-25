fn main() {
     // in rust, string used UTF-8 encoding
     // in the core,only one string 'cut' type: str (or &str). saved by 0&1
     // String is a heap-allocated string. saved by vector from standard library
     // String is growable, str is not. encoding by UTF-8
     // String can be converted to &str with & operator
    let mut s1: String = String::new();
    let s2: String = "initial contents".to_string();
    let s3: String = String::from("initial contents😎💖");
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}\n", s3);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // update string
    let mut s4: String = String::from("foo");
    let s5: String = "bar".to_string();
    s4.push_str(&s5);
    println!("s4 is {}", s4);
    s4.push('!');
    println!("s5 is {}", s5);
    println!("s4 new is {}", s4);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    let s6: String = String::from("Hello, ");
    let s7: String = String::from("world!");
    let s8: String = s6 + &s7;
    println!("s8 is {}", s8);
     /* '+'
        like this:
        fn add(self, s: &str) -> String {
            self.push_str(s);
            self
        }
     */
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}
