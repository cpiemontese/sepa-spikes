use actix_web::{post, web::Json, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::stripe::Stripe;

#[derive(Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct StripeCustomer {
    id: String,
}
#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    customer_id: String,
    amount: String,
    currency: String,
    payment_method: String,
}

#[post("/customers")]
pub async fn customer(customer: Json<Customer>, request: HttpRequest) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let stripe_customer: StripeCustomer = stripe
        .create_a_customer(&customer.name, &customer.email)
        .await
        .json()
        .await
        .unwrap();
    HttpResponse::Ok().json(stripe_customer)
}

#[post("/payment-intents")]
pub async fn create_payment_intent(
    payment_intent: Json<PaymentRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let stripe = req
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let stripe_customer: PaymentIntentCreated = stripe
        .create_a_payment_intent(
            payment_intent.customer_id.clone(),
            payment_intent.amount.clone(),
            payment_intent.currency.clone(),
            payment_intent.payment_method.clone(),
        )
        .await
        .json()
        .await
        .unwrap();
    HttpResponse::Ok().json(stripe_customer)
}
#[derive(Serialize, Deserialize)]
struct PaymentIntentCreated {
    id: String,
}
