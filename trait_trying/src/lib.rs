pub trait Summary {
    fn summarize(&self) -> String{
        String::from("(Read more...)")  // default implement
    }

    fn summarize_author(&self) -> String;

    fn summarize1(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
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