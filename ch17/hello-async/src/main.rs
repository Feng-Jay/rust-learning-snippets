use std::time::{Duration, Instant};
use trpl::{Html, Either};
use std::pin::Pin;
use std::pin::pin;
use std::thread;
use trpl::{StreamExt, Stream, ReceiverStream};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html()); 
    return (url, title);
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn timeout <F: Future> (
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(max_time),
    }
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move{
        let messages = vec!["a", "b", "c", "d", "e"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            if let Err(send_error) = tx.send(format!("Message: {}", message)) {
                eprintln!("Failed to send message: {}", send_error);
                break;
            }
        }
    });
    
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Failed to send interval: {}", send_error);
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn main() {

    trpl::run(
        async{
            let messages = get_message().timeout(Duration::from_millis(200));
            let intervals = get_intervals()
                .map(|count| format!("Interval: {count}"))
                .throttle(Duration::from_millis(100))
                .timeout(Duration::from_secs(10));
            let merged = messages.merge(intervals).take(20);
            let mut stream = pin!(merged);
            while let Some(result) = stream.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                }
            }
        }
    );

    trpl::run(async {
        let mut messages = pin!(get_message().timeout(Duration::from_millis(200)));
        while let Some(message) = messages.next().await {
            match message {
                Ok(msg) => println!("Received: {}", msg),
                Err(reason) => {
                    eprintln!("Problem: {reason:?}");
                }
            }
        }
    });

    trpl::run(async {
        let v = vec![1, 2, 3, 4, 5];
        let iter = v.iter().map(|n| n*2);
        let mut stream = trpl::stream_from_iter(iter);
        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        while let Some(value) = filtered.next().await {
            println!("got value {value}");
        }
    });

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });

    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });

    trpl::run(
        async{
            let one_ns = Duration::from_nanos(1);
            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::sleep(one_ns).await;
                }
            }.await;
            let time = Instant::now() - start;
            println!("Sleeping 1000 times for 1ns took {:?}", time);

            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::yield_now().await;
                }
            }.await;
            let time = Instant::now() - start;
            println!("Yielding 1000 times took {:?}", time);
        }
    );

    

    // let args: Vec<String> = std::env::args().collect();
    // // async {
    // //     let _ = page_title(&args[1]).await;
    // // };
    // trpl::run(
    //     async {
    //         let title1 = page_title(&args[1]);
    //         let title2 = page_title(&args[2]);
    //         let (url, maybe_title) = 
    //             match trpl::race(title1, title2).await {
    //                 Either::Left(left) => left,
    //                 Either::Right(right) => right,
    //             };
    //         println!("{url} returned first");
    //         match maybe_title {
    //             Some(title) => println!("Its page title is: '{title}'"),
    //             None => println!("Its title could not be parsed."),
    //         }
    //     }
    // );
    trpl::run(async {

        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        let tx1_fut = async move {
            println!("starting to send values...\n");
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            println!("starting to receive values...\n");
            while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }};

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(700)).await;
            }
        };

        // trpl::join3(rx_fut, tx_fut, tx1_fut).await;
        let iter: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(rx_fut), Box::pin(tx_fut), Box::pin(tx1_fut)];
        trpl::join_all(iter).await;

        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        };
        // trpl::join(fut1, fut2).await;
        // handle.await.unwrap();
    });
}
