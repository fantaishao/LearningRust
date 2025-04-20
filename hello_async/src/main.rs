use trpl::{Either, Html};
use std::time::Duration;
use std::thread;

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        // let title_fut_1 = page_title(&args[1]);
        // let title_fut_2 = page_title(&args[2]);

        // let (url, maybe_title) =
        //     match trpl::race(title_fut_1, title_fut_2).await {
        //         Either::Left(left) => left,
        //         Either::Right(right) => right,
        //     };

        // println!("{url} returned first");
        // match maybe_title {
        //     Some(title) => println!("Its page title is: '{title}'"),
        //     None => println!("Its title could not be parsed."),
        // }

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // fut1.await; // this will complete before moving to fut2
        // handle.await.unwrap();

        // let fut2 = async { 
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        //  };
        // fut2.await; // this will run after fut1 completes
        
        //trpl::join(fut1, fut2).await;

        fut1.await;

        let (tx, mut rx) = trpl::channel();
        
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

        while let Some(value) = rx.recv().await {
            println!("received: {value}");
        }
    })
}

// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let text = trpl::get(url).await.text().await;
//     let title = Html::parse(&text)
//         .select_first("title")
//         .map(|title| title.inner_html());
//     (url, title)
// }