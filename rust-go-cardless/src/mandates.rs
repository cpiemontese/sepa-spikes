use actix_web::{get, post, web::Json, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::gocardless::GoCardless;

#[post("/mandates")]
pub async fn create(mandate: Json<Mandate>, request: HttpRequest) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let result = go_cardless.create_mandate(mandate.0).await;
    HttpResponse::Ok().body(result.unwrap())
}

#[get("/mandates")]
pub async fn list(request: HttpRequest) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let result = go_cardless.get_all_mandate().await.unwrap();
    HttpResponse::Ok().body(result)
}

#[derive(Deserialize, Serialize)]
pub struct Mandates {
    pub mandates: Mandate,
}

#[derive(Deserialize, Serialize)]
pub struct Mandate {
    links: MandateLinks,
}

#[derive(Deserialize, Serialize)]
struct MandateLinks {
    customer_bank_account: String,
}
