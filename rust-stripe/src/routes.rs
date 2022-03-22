use std::collections::HashMap;

use actix_web::{get, post, HttpResponse, Responder};
use reqwest::Url;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/customer")]
pub async fn customer(req_body: String) -> impl Responder {
    let client = reqwest::Client::new();
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let stripe_url = Url::parse("https://api.stripe.com/v1/").unwrap();
    create_a_customer(&client, &stripe_url, &secret_key).await;
    // let response = create_a_payment_intent(client, stripe_url, secret_key, "cus_LMUT6c5j1u8ubB").await;
    // dbg!(response.text().await);

    HttpResponse::Ok().body(req_body)
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
