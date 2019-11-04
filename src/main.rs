#[macro_use]
extern crate serde_derive;

mod github; // use github is not legal
mod hackernews;
mod reddit;

use futures::future;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // concurrently crawl all sites
    match future::join3(github::crawl(), reddit::crawl(), hackernews::crawl()).await {
        (Ok(_), Ok(_), Ok(_)) => println!("crawl success"),
        (a, b, c) => {
            println!("github resp : {:?}", a);
            println!("reddit resp : {:?}", b);
            println!("hackernews resp : {:?}", c);
        }
    }

    Ok(())
}
