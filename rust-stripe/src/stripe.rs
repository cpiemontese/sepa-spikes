use std::collections::HashMap;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
#[derive(Clone)]
pub struct Stripe {
    client: Client,
    stripe_url: Url,
    secret_key: String,
}

impl Stripe {
    pub fn new(stripe_url: Url, secret_key: String) -> Self {
        Self {
            client: Client::new(),
            stripe_url,
            secret_key,
        }
    }

    pub async fn get_customers(
        &self
    ) -> reqwest::Response {
        let resp = self
            .client
            .get(self.stripe_url.join("customers").unwrap())
            .basic_auth::<&str, String>(&self.secret_key, None)
            .send()
            .await
            .unwrap();

        resp
    }


    pub async fn subscribe(
        &self,
        customer_id: &str,
        price_id: &str,
    ) -> reqwest::Response {
        let form_data = HashMap::from([
            ("customer", customer_id),
            ("items[0][price]", price_id),
            ("expand[0]","latest_invoice.payment_intent"),
        ]);
        let resp = self
            .client
            .post(self.stripe_url.join("subscriptions").unwrap())
            .form(&form_data)
            .basic_auth::<&str, String>(&self.secret_key, None)
            .send()
            .await
            .unwrap();

        resp
    }

    pub async fn create_a_customer(
        &self,
        customer_name: &str,
        customer_email: &str,
    ) -> reqwest::Response {
        let form_data = HashMap::from([
            ("name".to_string(), customer_name),
            ("email".to_string(), customer_email),
        ]);
        let resp = self
            .client
            .post(self.stripe_url.join("customers").unwrap())
            .form(&form_data)
            .basic_auth::<&str, String>(&self.secret_key, None)
            .send()
            .await
            .unwrap();

        resp
    }

    pub async fn create_a_product(&self, product_name: &str, product_price: &str) -> Price {
        let product_form_data = HashMap::from([("name".to_string(), product_name)]);

        let product: Product = self
            .client
            .post(self.stripe_url.join("products").unwrap())
            .form(&product_form_data)
            .basic_auth::<&str, String>(&self.secret_key, None)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let fixed_price: f32 = product_price.parse().unwrap();

        let price_form_data = HashMap::from([
            ("product".to_string(), product.id),
            (
                "unit_amount".to_string(),
                ((fixed_price * 100_f32) as i32).to_string(),
            ),
            ("currency".to_string(), "eur".to_string()),
            ("recurring[interval]".to_string(), "day".to_string()),
        ]);

        let price: Price = self
            .client
            .post(self.stripe_url.join("prices").unwrap())
            .form(&price_form_data)
            .basic_auth::<&str, String>(&self.secret_key, None)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        price
    }

    pub async fn create_a_setup_intent(
        &self,
        customer_id: String,
        payment_method: String,
    ) -> reqwest::Response {
        let form_data = HashMap::from([
            ("customer".to_string(), customer_id.to_string()),
            ("payment_method_types[]".to_string(), payment_method),
        ]);
        let resp = self
            .client
            .post(self.stripe_url.join("setup_intents").unwrap())
            .form(&form_data)
            .basic_auth::<String, String>(self.secret_key.clone(), None)
            .send()
            .await;

        resp.unwrap()
    }

  pub async fn set_payment_method_as_default(
        &self,
        payment_method: String,
        customer_id: String,
    ) -> reqwest::Response {
        let form_data = HashMap::from([
            (
                "invoice_settings[default_payment_method]".to_string(),
                payment_method.to_string(),
            ),
        ]);
        let resp = self
            .client
            .post(self.stripe_url.join(&format!("customers/{}", customer_id)).unwrap())
            .form(&form_data)
            .basic_auth::<String, String>(self.secret_key.clone(), None)
            .send()
            .await;

        resp.unwrap()
    }

    pub async fn create_a_payment_intent(
        &self,
        customer_id: String,
        amount: String,
        currency: String,
        payment_method: String,
    ) -> reqwest::Response {
        let form_data = HashMap::from([
            ("customer".to_string(), customer_id.to_string()),
            ("currency".to_string(), currency),
            ("amount".to_string(), amount),
            ("payment_method_types[]".to_string(), payment_method),
            (
                "metadata[integration_checker]".to_string(),
                "sepa_debit_accept_a_payment".to_string(),
            ),
            ("setup_future_usage".to_string(), "off_session".to_string()),
        ]);
        let resp = self
            .client
            .post(self.stripe_url.join("payment_intents").unwrap())
            .form(&form_data)
            .basic_auth::<String, String>(self.secret_key.clone(), None)
            .send()
            .await;

        resp.unwrap()
    }
}

#[derive(Serialize, Deserialize)]
struct Product {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Price {
    pub id: String,
}
