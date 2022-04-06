mod customers;
mod gocardless;
mod mandates;
mod payments;
mod refunds;

use actix_files::Files;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

use gocardless::GoCardless;

// fn somma(x: u16, f: impl FnOnce(u16) -> u16) -> impl FnOnce(u16) -> u16 {
//     move |y| f(x + y)
// }
// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn print_me(x: impl Debug) {
//     println!("{x:?}");
// }

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
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
            .service(payments::list)
            .service(refunds::refund)
            .service(Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
