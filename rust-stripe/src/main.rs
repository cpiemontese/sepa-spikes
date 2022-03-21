extern crate dotenv;

use dotenv::dotenv;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let stripe_url = Url::parse("https://api.stripe.com/v1/")?;

    let client = reqwest::Client::new();
    let resp = client
        .post(stripe_url.join("customers")?)
        .basic_auth::<String, String>(secret_key, None)
        .send()
        .await?;
    println!("{:#?}", resp.json().await?);

    Ok(())
}
