// defining a trait
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // default implementation
    }

    fn summarize_author(&self) -> String;
}

// implementing a trait on a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let post = SocialPost{
        username: String::from("Bob"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());
    println!("1 new social post: {}", post.summarize_author());
}

// trait as parameter
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

// return type that implement traits
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceberg"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            )
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
}