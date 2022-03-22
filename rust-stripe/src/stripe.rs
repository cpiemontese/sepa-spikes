use std::collections::HashMap;

use reqwest::{Client, Url};
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
