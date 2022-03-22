extern crate dotenv;

mod routes;

use std::collections::HashMap;

use actix_files::Files;
use dotenv::dotenv;
use reqwest::Url;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::customer)
            .service(Files::new("/public", "./public").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
