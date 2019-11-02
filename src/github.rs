/// Get github trending list
/// support configure for interested languages
use reqwest;
use serde_json::Value;

// https://developer.github.com/v3/
// curl -G https://api.github.com/search/repositories --data-urlencode "sort=stars" --data-urlencode "order=desc" --data-urlencode "q=language:java"  --data-urlencode "q=created:>`date -v-7d '+%Y-%m-%d'`"
pub(crate) async fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let params = [
        ("s", "stars"),
        ("o", "desc"),
        // a month ago or a week ago
        ("q", "created:>2019-05-01 language:Rust"),
    ];

    let body = client
        .get("http://api.github.com/search/repositories")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    // println!("total repo count : {}", j["total_count"]);

    let j: Value = serde_json::from_str(body.as_str())?;
    for item in j["items"].as_array().unwrap() {
        // todo, beautiful table
        println!(
            "item : github.com/{}, {}, ",
            item["full_name"].as_str().unwrap(),
            item["stargazers_count"]
        );
    }

    Ok(())
}
