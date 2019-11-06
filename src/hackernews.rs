use reqwest;
use std::time::Duration;
use prettytable::{Table, Row, Cell};

//use serde_json::Value;

//use serde::{Deserialize, Serialize};

/*
    {
    "by" : "dhouston",
    "descendants" : 71,
    "id" : 8863,
    "kids" : [ 8952, 9224, 8917, 8884, 8887, 8943, 8869, 8958, 9005, 9671, 8940, 9067, 8908, 9055, 8865, 8881, 8872, 8873, 8955, 10403, 8903, 8928, 9125, 8998, 8901, 8902, 8907, 8894, 8878, 8870, 8980, 8934, 8876 ],
    "score" : 111,
    "time" : 1175714200,
    "title" : "My YC app: Dropbox - Throw away your USB drive",
    "type" : "story",
    "url" : "http://www.getdropbox.com/u/2/screencast.html"
    }
*/
#[derive(Serialize, Deserialize, Debug)]
struct Story {
    by: String,
    descendants: i64,
    id: i64,
    //kids : Vec<i64>,
    score: i64,
    time: i64,
    title: String,
    r#type: String,
    url: String,
}

// https://github.com/HackerNews/API
pub(crate) async fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    // before hacker news there is gfw
    let client = reqwest::Client::builder()
        .use_sys_proxy()
        .timeout(Duration::from_secs(5))
        .build()?;

    let json = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?
        .text()
        .await?;

    let j: Vec<i64> = serde_json::from_str(json.as_str())?;

    println!("hackernews top stories count: {}", j.len());

    let mut story_list = vec![];
    for i in 0..10 {
        let json = client
            .get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", j[i]).as_str())
            .send()
            .await?
            .text()
            .await?;

        let story: Story = serde_json::from_str(json.as_str()).unwrap();
        story_list.push(story);
    }

    /* 这种写法是不行的，闭包没有返回值
    而 async await 又需要在出错时有返回值
    (0..10).for_each(|i|{});
    总之就是编译过不了。
    */
    let mut table = Table::new();
    story_list.iter().for_each(|s|{
        table.add_row(row![
            Fy -> s.title,
        ]);
        table.add_row(row![
            s.url,// s.by,
        ]);
    });

    table.printstd();

    //println!("story list :{:?}", story_list);
    Ok(())
}
