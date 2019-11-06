use std::time::Duration;
use prettytable::{Table, Row, Cell};

/// Get reddit news for subscribed channels

#[derive(Serialize, Deserialize, Debug)]
struct RedditResp {
    kind: String,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    data: BlockDetail,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockDetail {
    title: String,
    selftext: String,
    permalink: String,
    url: String,
    created_utc: f64,
    author: String,
    ups: i32,
    subreddit_name_prefixed: String,
}

// https://www.reddit.com/dev/api/
// www.reddit.com/r/rust.json
// www.reddit.com/r/golang.json
pub(crate) async fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .use_sys_proxy()
        .timeout(Duration::from_secs(5))
        .build()?;

    let body = client
        .get("https://www.reddit.com/r/rust.json")
        .send()
        .await?
        .text()
        .await?;

    let j: RedditResp = serde_json::from_str(body.as_str())?;

    let mut table = Table::new();
    j.data.children.iter().for_each(|x| {
        table.add_row(row![
            Fy -> x.data.title,
        ]);
        table.add_row(row![
            format!("reddit.com/{}", x.data.permalink),
        ]);
    });

    table.printstd();

    Ok(())
}
