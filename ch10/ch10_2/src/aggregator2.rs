use structs::{ Tweet, NewsArticle };

/*
we could define the Summary trait to have a summarize_author method
whose implementation is required, and then define a summarize method
that has a default implementation that calls the summarize_author method:
*/
pub trait Summary {
    fn summarize_author(&self) -> String;

    // abstract method を default method で使用する
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn listing_10_14_2() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("Listing 10-14-2: 1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}

// Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Traits as Parameters: Breaking news! {}", item.summarize());
}
