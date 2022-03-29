use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::gocardless::GoCardless;

#[post("/payments")]
pub async fn pay(payment: Json<Payment>, request: HttpRequest) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let result = go_cardless.pay(payment.0).await.unwrap();
    HttpResponse::Ok().body(result)
}

#[derive(Serialize, Deserialize)]
pub struct Payments {
    pub payments: Payment,
}

#[derive(Serialize, Deserialize)]
pub struct Payment {
    amount: String,
    currency: String,
    links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    mandate: String,
}
