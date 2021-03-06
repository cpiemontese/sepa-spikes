extern crate dotenv;

mod routes;
mod stripe;
mod webhook;
mod enums;

use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use reqwest::Url;
use stripe::Stripe;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let secret_key =
            std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
        let stripe_url = Url::parse("https://api.stripe.com/v1/").unwrap();
        let stripe = Stripe::new(stripe_url, secret_key);

        App::new()
            .app_data(stripe)
            .service(routes::customer)
            .service(routes::customers)
            .service(routes::subscriptions)
            .service(routes::set_default_payment)
            .service(routes::product)
            .service(routes::prices)
            .service(routes::create_payment_intent)
            .service(routes::setup_intents)
            .service(webhook::webhook)
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
