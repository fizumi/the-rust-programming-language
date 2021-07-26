use structs::{ Tweet, NewsArticle };

// Listing 10-12: A Summary trait that consists of the behavior provided by
//                a summarize method
pub trait Summary {
    // fn summarize(&self) -> String; // no default implementations

    // Listing 10-14: Definition of a Summary trait with
    //                a default implementation of the summarize method
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn listing_11_13() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("Listing 10-14:1 new tweet: {}", tweet.summarize());
}

// https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html
// Lifetime Annotations in Struct Definitions
pub struct NewsArticle2<'a> {
    pub headline: &'a String,
    pub location: &'a String,
    pub author: &'a String,
    pub content: &'a String,
}
impl Summary for NewsArticle2<'_> {} // use default implementation of Summary

pub fn listing_10_14() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    let NewsArticle { headline, location, author, content } = &article;
    let article2 = NewsArticle2 { headline, location, author, content };

    println!("Listing 10-14: New article available! {}", article.summarize());
    println!("Listing 10-14: New article available! {}", article2.summarize());
}

