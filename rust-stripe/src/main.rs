extern crate dotenv;

use std::{collections::HashMap, sync::Arc};

use dotenv::dotenv;
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let stripe_url = Url::parse("https://api.stripe.com/v1/")?;

    let client = reqwest::Client::new();
    create_a_customer(&client, &stripe_url, &secret_key).await;
    let response =
        create_a_payment_intent(client, stripe_url, secret_key, "cus_LMUT6c5j1u8ubB").await;
    dbg!(response.text().await);
    Ok(())
}

async fn create_a_customer(
    client: &reqwest::Client,
    stripe_url: &Url,
    secret_key: &str,
) -> reqwest::Response {
    let form_data = HashMap::from([
        ("description".to_string(), "A customer".to_string()),
        ("name".to_string(), "Arturo".to_string()),
    ]);
    let resp = client
        .post(stripe_url.join("customers").unwrap())
        .form(&form_data)
        .basic_auth::<&str, String>(secret_key, None)
        .send()
        .await
        .unwrap();

    resp
}
async fn create_a_payment_intent(
    client: reqwest::Client,
    stripe_url: Url,
    secret_key: String,
    customer_id: &str,
) -> reqwest::Response {
    let form_data = HashMap::from([
        ("customer".to_string(), customer_id.to_string()),
        ("currency".to_string(), "eur".to_string()),
        ("amount".to_string(), "1000".to_string()),
        (
            "payment_method_types[]".to_string(),
            "sepa_debit".to_string(),
        ),
        (
            "metadata[integration_checker]".to_string(),
            "sepa_debit_accept_a_payment".to_string(),
        ),
        ("setup_future_usage".to_string(), "off_session".to_string()),
    ]);
    let resp = client
        .post(stripe_url.join("payment_intents").unwrap())
        .form(&form_data)
        .basic_auth::<String, String>(secret_key, None)
        .send()
        .await;

    resp.unwrap()
}
