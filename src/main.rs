// Trait(共通の振る舞い)を定義する
pub trait Summary {
    fn summarize(&self) -> String;
    fn get_author(&self) -> String;
}

// データ構造を定義する
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// SummaryトレイトをNewsArticleとTweetに実装する
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn get_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }

    fn get_author(&self) -> String {
        format!("{}", self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rustはいいね"),
        location: String::from("東京"),
        author: String::from("田中"),
        content: String::from("Rustは非常に便利です！"),
    };

    let tweet = Tweet {
        username: String::from("佐藤"),
        content: String::from("今日の天気はいいですね！"),
        reply: false,
        retweet: false,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
