use actix_web::{
    get, post,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::gocardless::{BankAccount, GoCardless};

#[get("/customers")]
pub async fn list(request: HttpRequest) -> HttpResponse {
    let customer = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected")
        .get_all_customers()
        .await;
    HttpResponse::Ok().body(customer.unwrap())
}

#[get("/customers/{id}")]
pub async fn get_customer(id: Path<String>, request: HttpRequest) -> HttpResponse {
    let id = id.into_inner();
    let customer = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected")
        .get_customer(id)
        .await;
    HttpResponse::Ok().body(customer.unwrap())
}

#[post("/customers")]
pub async fn create(customer: Json<Customer>, request: HttpRequest) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let customer = go_cardless.create_customer(customer.0).await;
    HttpResponse::Ok().body(customer.unwrap())
}

#[get("/customer_bank_accounts")]
pub async fn list_bank_accounts(request: HttpRequest) -> HttpResponse {
    let customer = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected")
        .get_all_customer_bank_accounts()
        .await;
    HttpResponse::Ok().body(customer.unwrap())
}

#[post("/customer_bank_accounts")]
pub async fn add_bank_account(
    bank_account: Json<BankAccount>,
    request: HttpRequest,
) -> HttpResponse {
    let go_cardless = request
        .app_data::<GoCardless>()
        .expect("A client go cardless is expected");

    let bank_account = go_cardless.add_bank(bank_account.0).await;
    HttpResponse::Ok().body(bank_account.unwrap())
}

#[derive(Deserialize, Serialize)]
pub struct Customer {
    email: String,
    given_name: String,
    family_name: String,
    address_line1: Option<String>,
    address_line2: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country_code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Customers {
    pub customers: Customer,
}
