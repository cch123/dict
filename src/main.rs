// use futures::join; 需要在 toml 里开启 feature gate 才能用

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate prettytable;

mod github; // use github is not legal
mod hackernews;
mod reddit;

use future::Future;
use futures::{future, StreamExt};
use std::pin::Pin;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // concurrently crawl all sites
    // 第一种写法
    match future::join3(github::crawl(), reddit::crawl(), hackernews::crawl()).await {
        (Ok(_), Ok(_), Ok(_)) => println!("crawl success"),
        (a, b, c) => {
            println!("github resp : {:?}", a);
            println!("reddit resp : {:?}", b);
            println!("hackernews resp : {:?}", c);
        }
    }

    /*
    第二种写法
    let result_list = future::join_all(vec![
        Box::pin(github::crawl())
            as Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>,
        Box::pin(reddit::crawl())
            as Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>,
        Box::pin(hackernews::crawl())
            as Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>>>>,
    ]).await;

    result_list.iter().for_each(|result|{
        println!("{:?}", result);
    });
    */

    // 第三种写法
    /*
    match futures::join!(github::crawl(), reddit::crawl(), hackernews::crawl()) {
        (Ok(_), Ok(_), Ok(_)) => {
            println!("it is fucking ok");
        }
        (a,b,c) => {
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", c);
        }
    }
    */
    Ok(())
}
