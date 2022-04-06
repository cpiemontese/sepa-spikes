use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::gocardless::GoCardless;

#[post("/refunds")]
pub async fn refund(refund: Json<Refund>, request: HttpRequest) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let result = go_cardless.refund(refund.0).await.unwrap();
    HttpResponse::Ok().body(result)
}

#[derive(Serialize, Deserialize)]
pub struct Refunds {
    pub refunds: Refund,
}

#[derive(Serialize, Deserialize)]
pub struct Refund {
    amount: String,
    links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    payment: String,
}
