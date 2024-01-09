use std::ffi::CString;

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
    let s8: String = s8 + &s7;
    println!("s8 is {}, s7 is {}", s8, s7);
     /* '+'
        like this:
        fn add(self, s: &str) -> String {
            self.push_str(s);
            self
        }
     */
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // format! macro
    let s1: String = String::from("tic");
    let s2: String = String::from("tac");
    let s3: String = String::from("toe");

     // let s3: String = s1 + "-" + &s2 + "-" + &s3;
     // println!("s3 is {}", s3);

    let s4: String = format!("{}-{}-{}", s1, s2, s3);
    println!("s4 is {}", s4);
     // let s3: String = format!("{}-{}-{}", s1, s2, s3);
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
     // index string
    let s1: String = String::from("hello");
    let s2: String = String::from("你好");
    let s3: String = String::from("👋🏻");
    let len1: usize = s1.len();
    let len2: usize = s2.len();
    let len3: usize = s3.len();
    println!("{} len is {}\n{} len is {}\n{} len is {}", s1, len1, s2, len2, s3, len3);
     // unicode scalar value
     // let h: char = s1[0]; // error
     // println!("h is {}", h);
    let answer = &s2[0..3]; // get a cut of string s2
    println!("answer is {}", answer);

    let h: char = s1.chars().nth(0).unwrap();
    println!("h is {}", h);

    let h: char = s2.chars().nth(0).unwrap();
    println!("h is {}", h);

    for b in s2.bytes() {
        println!("{}", b);
    }
    for b in s2.chars() { // unicode scalar value
        println!("{}", b);
    }
}
