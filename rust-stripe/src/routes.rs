use actix_web::{
    get, post,
    web::{Json, Path},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::stripe::{self, Stripe};

#[derive(Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct Product {
    pub name: String,
    pub price: String,
}

#[derive(Serialize, Deserialize)]
struct StripeCustomer {
    id: String,
}
#[derive(Serialize, Deserialize)]
pub struct SetupIntentCreated {
    id: String,
    client_secret: String,
}
#[derive(Serialize, Deserialize)]
pub struct SetupIntentRequest {
    customer_id: String,
    payment_method: String,
}
#[derive(Serialize, Deserialize)]
pub struct DefaultPayment {
    payment_method: String,
}
#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    customer_id: String,
    amount: String,
    currency: String,
    payment_method: String,
}

#[derive(Serialize, Deserialize)]
pub struct Subscription {
    customer_id: String,
    price_id: String,
}

#[get("/customers")]
pub async fn customers(request: HttpRequest) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let customers = stripe.get_customers().await.text().await.unwrap();

    HttpResponse::Ok().body(customers)
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

#[post("/subscriptions")]
pub async fn subscriptions(subscription: Json<Subscription>, request: HttpRequest) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let subscription_result = stripe
        .subscribe(&subscription.customer_id, &subscription.price_id)
        .await
        .text()
        .await
        .unwrap();
    HttpResponse::Ok().body(subscription_result)
}
#[post("/customers/{id}")]
pub async fn set_default_payment(
    path: Path<String>,
    default_payment: Json<DefaultPayment>,
    request: HttpRequest,
) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let customer_id = path.into_inner();
    let response = stripe
        .set_payment_method_as_default(default_payment.payment_method.to_string(), customer_id)
        .await;

    HttpResponse::Ok().body(response.text().await.unwrap())
}

#[post("/setup-intents")]
pub async fn setup_intents(
    setup_intent: Json<SetupIntentRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let stripe = req
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let stripe_customer: SetupIntentCreated = stripe
        .create_a_setup_intent(
            setup_intent.customer_id.clone(),
            setup_intent.payment_method.clone(),
        )
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

#[get("/prices")]
pub async fn prices(request: HttpRequest) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let prices = stripe.get_prices().await.text().await.unwrap();

    HttpResponse::Ok().body(prices)
}

#[post("/products")]
pub async fn product(product: Json<Product>, request: HttpRequest) -> HttpResponse {
    let stripe = request
        .app_data::<Stripe>()
        .expect("A client stripe is expected");

    let stripe_price: stripe::Price = stripe.create_a_product(&product.name, &product.price).await;
    HttpResponse::Ok().json(stripe_price)
}
#[derive(Serialize, Deserialize)]
struct PaymentIntentCreated {
    id: String,
    customer: String,
    client_secret: String,
}
