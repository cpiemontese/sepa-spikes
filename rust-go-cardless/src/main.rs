mod customers;
mod gocardless;
mod mandates;
mod payments;

use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use gocardless::GoCardless;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let secret_key =
            std::env::var("GO_CARDLESS_TOKEN").expect("Missing GO_CARDLESS_TOKEN in env");

        let client = GoCardless::new(format!("Bearer {secret_key}"));

        App::new()
            .app_data(client)
            .service(customers::create)
            .service(customers::add_bank_account)
            .service(customers::list)
            .service(customers::list_bank_accounts)
            .service(customers::get_customer)
            .service(mandates::create)
            .service(mandates::list)
            .service(payments::pay)
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
