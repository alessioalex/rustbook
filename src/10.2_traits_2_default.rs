// Definition of a Summary trait with a default implementation of the summarize method
pub trait Summary {
    // fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
        // format!("(Read more from{}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
