use trpl::Html;

fn main() {
    println!("async!");

    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title3(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

use std::future::Future;

fn page_title3(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

// $  cargo run https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html
// result: The title for https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html was Futures and the Async Syntax - The Rust Programming Language