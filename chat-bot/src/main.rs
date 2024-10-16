
use reqwest::Client;
use reqwest_oauth1::OAuthClientProvider;
use scraper::{Html, Selector};
use dotenv::dotenv;
use std::env;
use serde_json::json;
use std::error::Error;
use tokio;


fn split_into_chunks(text: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let mut end = start + chunk_size;

        if end >= text.len() {
            end = text.len();
        }

        if end < text.len() {
            if let Some(last_space) = text[start..end].rfind(' ') {
                end = start + last_space;
            }
        }

        let chunk = text[start..end].to_string();
        chunks.push(chunk);

        start = end + 1;
    }
    chunks
}

async fn post_tweet_v2(
    consumer_key: &str,
    consumer_secret: &str,
    access_token: &str,
    access_token_secret: &str,
    tweet_text: &str,
) -> Result<(), Box<dyn Error>> {

    let url = "https://api.twitter.com/2/tweets";
    println!("here2");
    let tweet_potential = split_into_chunks(tweet_text, 280);
    let tweet_text = &tweet_potential[0];
    println!("{:?}", tweet_text);
    let tweet = json!({
        "text": tweet_text.to_string(),
    });
    let secrets = reqwest_oauth1::Secrets::new(consumer_key, consumer_secret).token(access_token, access_token_secret);



    let client = Client::new();
    let body = serde_json::to_string(&tweet)?;


    let response = client
    .oauth1(secrets.clone())
    .post(url)
    .header("Content-Type", "application/json") // Set the content type header to JSON
    .body(body)
    .send().await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eprintln!("Failed to post tweet: {}", error_text);
        return Err("Failed to post the tweet".into())
    }

    let response_json: serde_json::Value = response.json().await?;

    let mut previous_tweet_id = response_json["data"]["id"]
        .as_str()
        .ok_or("Failed to extract tweet ID")?
        .to_string();

    for tweets in &tweet_potential[1..] {
        let tweet = json!({
            "text": tweets,
            "reply": {
                "in_reply_to_tweet_id": previous_tweet_id,
            }
        });

        let body = serde_json::to_string(&tweet)?;

        let client2= Client::new();
        let response = client2
            .oauth1(secrets.clone())
            .post(url)
            .header("Content-Type", "application/json")
            .body(body) 
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            eprintln!("Failed to post reply: {}", error_text);
            return Err("Failed to post the reply tweet".into());
        }

        let response_json: serde_json::Value = response.json().await?;
        previous_tweet_id = response_json["data"]["id"]
            .as_str()
            .ok_or("Failed to extract tweet ID")?
            .to_string();

        println!("Reply posted successfully with ID: {}", previous_tweet_id);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let body = reqwest::get("https://www.meteo.be/fr/ixelles").await?.text().await?;
    let weather_body = Html::parse_document(&body);
    let selector = Selector::parse("now-cast").unwrap();
    let mut tweet_content =String::new(); 

    for elem in weather_body.select(&selector){
        let body_elems = elem.text().collect::<Vec<_>>();
        tweet_content.push_str(body_elems[0]);
        tweet_content.push_str(" ");
    }
    let key =env::var("DEEPL_AUTH_KEY")?;
    let client = Client::new();
    let data = json!({
        "text":[tweet_content],
        "target_lang":"EN"
    });
    let response = client.post("https://api-free.deepl.com/v2/translate")
    .header("Authorization", format!("DeepL-Auth-Key {}", key))
    .header("Content-Type", "application/json")
    .json(&data)
    .send().await?;

    let english_text = if response.status().is_success() {
        response.text().await?
    } else {
        eprintln!("Failed to translate. Status: {}", response.status());
        String::new()
    };
    let parsed_json:serde_json::Value = serde_json::from_str(&english_text)?;
    let english_tweet= parsed_json["translations"][0]["text"].as_str().unwrap_or("No weather today");
    println!("english tweet: {:?}", english_tweet);
    let consumer_key = env::var("CONSUMER_KEY")?;
    let consumer_secret = env::var("CONSUMER_SECRET")?;
    let access_token = env::var("OAUTH_TOKEN")?;
    let access_token_secret = env::var("OAUTH_TOKEN_SECRET")?;
    println!("here");
    post_tweet_v2(&consumer_key, &consumer_secret, &access_token, &access_token_secret, english_tweet).await?;
    
   //println!("English {:?}", english_tweet);

    Ok(())
}