use reqwest;
mod github; // use github is not legal
mod reddit;
mod hackernews;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://xargin.com")
        .await?
        .text()
        .await?;

    //println!("{}", body);

    github::crawl().await?;
    reddit::crawl();
    hackernews::crawl();

    Ok(())
}
