/// Get github trending list
/// support configure for interested languages
use reqwest;

// https://developer.github.com/v3/
// curl -G https://api.github.com/search/repositories --data-urlencode "sort=stars" --data-urlencode "order=desc" --data-urlencode "q=language:java"  --data-urlencode "q=created:>`date -v-7d '+%Y-%m-%d'`"
pub(crate) async fn crawl() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "crawl github");
    let client = reqwest::Client::new();
    let params = [
        ("sort", "stars"),
        ("order", "desc"),
        ("q", "language:rust"),
        ("q", "created>2019-11-01"),
    ];

    let body = client.get("http://api.github.com/search/repositories")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;
    println!("github body: {}", body);
    Ok(())
}
