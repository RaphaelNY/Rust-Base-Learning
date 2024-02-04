use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")  // default implement
    }

    fn summarize_author(&self) -> String;

    fn summarize1(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


pub struct Pair<T> {
    x: T,
    y: T,
} impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
} impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
} impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //   format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

}// if this didn't implement the summarize method, it will use the default implement.

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
} impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


 // impl trait
 // notify function can get any input that achieved Summary trait.
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify2(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

 // trait bounds
 // - impl trait is grammar sugar of trait bounds.
 // - use '+' can ordered more than one trait.
 // - use 'where' can make the code more readable.
pub fn notifys<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifys1<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notifys2<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifys3<T>(item: T)
    where T: Summary + Display
{
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}{}", a.summarize(), b.summarize())
}

pub fn notifys4<T, U>(a: T, b: U) -> String
    where T: Summary + Display,
          U: Clone + Debug
{
    format!("{}{}", a.summarize(), b.summarize())
}

 // use trait as the return type
 // - impl trait. however the return type is a certain type, if you return a different type, it will be an error.
 //
pub fn notifyss(flag: bool) -> impl Summary {
     if flag {
         NewsArticle {
             headline: String::from("Penguins win the Stanley Cup Championship!"),
             location: String::from("Pittsburgh, PA, USA"),
             author: "".to_string(),
             content: "".to_string(),
         }
     } else {
         Tweet {
             username,
             content: "".to_string(),
             reply: false,
             retweet: false,
         }
     }
 }

 // you can coding for a type that if it achieved Atrait, it can achieve B trait, and following is example.
 // impl<T: Display + Clone> Summary for T {...}
 // in this statements, if input T achieved Display and Clone trait, it can achieve Summary trait.