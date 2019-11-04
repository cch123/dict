#[macro_use]
extern crate serde_derive;

mod github; // use github is not legal
mod hackernews;
mod reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    github::crawl().await?;
    hackernews::crawl().await?;
    reddit::crawl().await?;

    Ok(())
}
