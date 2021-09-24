use demo::Summary;
use demo::Tweet;
use demo::NewsArticle;

fn main() {
    // 实现 trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    
    // trait 的默认实现
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh, PA, USA"),
    };
    println!("1 new article: {}", article.summarize());
}
