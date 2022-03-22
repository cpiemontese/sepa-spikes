use std::collections::HashMap;

use actix_web::{get, post, web::Json, HttpResponse, Responder};
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct StripeCustomer {
    id: String,
}

#[post("/customer")]
pub async fn customer(customer: Json<Customer>) -> HttpResponse {
    let client = reqwest::Client::new();
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let stripe_url = Url::parse("https://api.stripe.com/v1/").unwrap();
    let stripe_customer: StripeCustomer = create_a_customer(
        &client,
        &stripe_url,
        &secret_key,
        &customer.name,
        &customer.email,
    )
    .await
    .json()
    .await
    .unwrap();
    // let response = create_a_payment_intent(client, stripe_url, secret_key, "cus_LMUT6c5j1u8ubB").await;
    // dbg!(response.text().await);

    HttpResponse::Ok().json(stripe_customer)
}

async fn create_a_customer(
    client: &reqwest::Client,
    stripe_url: &Url,
    secret_key: &str,
    customer_name: &str,
    customer_email: &str,
) -> reqwest::Response {
    let form_data = HashMap::from([
        ("name".to_string(), customer_name),
        ("email".to_string(), customer_email),
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
