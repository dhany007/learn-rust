use std::time::{Duration, Instant};
use std::pin::{Pin, pin};
use std::thread;
use trpl::Either;

fn main() {
    println!("Working with any number of futures!");

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("there"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });

    //  pass in multiple future types and produces a tuple of those types
    trpl::run(async {
        let a = async { 1u32 }; // implements Future<Output = u32>
        let b = async {"Hello"}; // implements Future<Output = &str>
        let c = async { true }; // implements Future<Output = bool>

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}\n{b_result}\n{c_result}");
    });

    println!("Racing futures");
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000{
                trpl::sleep(one_ns).await;
            }
        }
            .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds",
            time.as_secs_f32()
        );
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
            .await;
        let time = Instant::now() - start;
        println!(
            "'yield_now' version finished after {} seconds",
            time.as_secs_f32()
        );
    });

    println!("building our own async abstractions");
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })

}

fn slow (name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' run for {ms}ms");
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}