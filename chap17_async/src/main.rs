use trpl::{Either, Html};

fn main() {
    println!("async!");

    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        // who finish first between 2 url given
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first");


        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) ->  (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// $ cargo run https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html https://rust-lang.github.io/api-guidelines/naming.html
// result: https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html returned first
// Its page title is: 'Futures and the Async Syntax - The Rust Programming Language'
