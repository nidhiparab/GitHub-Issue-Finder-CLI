use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Issue{
    title: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct Resp{
    items: Vec<Issue>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let req_url = "https://api.github.com/search/issues?q=label:good-first-issue+state:open+language:Rust";

    let response = client
    .get(req_url)
    .header("User-Agent", "your-rust-cli")
    .send()
    .await?;
    
    let issues: Resp = response.json().await?;
    
    for issue in issues.items{
        println!("****************** \n");
        println!("{} \n {} \n", issue.title, issue.html_url);
    }
    println!("Hello, world!");
    Ok(())
}
