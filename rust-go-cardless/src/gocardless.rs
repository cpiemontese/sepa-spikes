use reqwest::{header, Client, Url};
use serde::{Deserialize, Serialize};

use crate::{
    customers::{Customer, Customers},
    payments::{Payment, Payments},
};

pub struct GoCardless {
    client: Client,
    base_url: Url,
}
#[derive(Deserialize, Serialize)]
pub struct BankAccount {
    iban: String,
    account_holder_name: String,
    currency: String,
    links: Links,
}

#[derive(Deserialize, Serialize)]
pub struct BankAccounts {
    customer_bank_accounts: BankAccount,
}

#[derive(Deserialize, Serialize)]
pub struct Links {
    customer: String,
}

#[derive(Deserialize, Serialize)]
struct Mandates {
    mandates: Mandate,
}

#[derive(Deserialize, Serialize)]
pub struct Mandate {
    links: MandateLinks,
}

#[derive(Deserialize, Serialize)]
struct MandateLinks {
    customer_bank_account: String,
}

impl GoCardless {
    pub fn new(token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&token).expect("No valid header"),
        );

        headers.insert(
            header::HeaderName::from_static("gocardless-version"),
            header::HeaderValue::from_static("2015-07-06"),
        );
        GoCardless {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .expect("Error creating client for GoCardless"),
            base_url: Url::parse("https://api-sandbox.gocardless.com").expect("Cannot parse url"),
        }
    }

    pub async fn create_customer(&self, customer: Customer) -> Result<String, reqwest::Error> {
        let url = self
            .base_url
            .join("/customers")
            .expect("Error creating url");

        let customers = Customers {
            customers: customer,
        };

        let result = self.client.post(url).json(&customers).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn pay(&self, payment: Payment) -> Result<String, reqwest::Error> {
        let url = self.base_url.join("/payments").expect("Error creating url");

        let payments = Payments { payments: payment };

        let result = self.client.post(url).json(&payments).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn create_mandate(&self, mandate: Mandate) -> Result<String, reqwest::Error> {
        let url = self.base_url.join("/mandates").expect("Error creating url");

        let mandates = Mandates { mandates: mandate };

        let result = self.client.post(url).json(&mandates).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn add_bank(&self, bank_account: BankAccount) -> Result<String, reqwest::Error> {
        let url = self
            .base_url
            .join("/customer_bank_accounts")
            .expect("Error creating url");

        let bank_accounts = BankAccounts {
            customer_bank_accounts: bank_account,
        };

        let result = self.client.post(url).json(&bank_accounts).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn get_all_customers(&self) -> Result<String, reqwest::Error> {
        let url = self
            .base_url
            .join("/customers")
            .expect("Error creating url");

        let result = self.client.get(url).send().await;
        let data = result.ok().unwrap();
        let response = data.text().await;
        response
    }

    pub async fn get_all_customer_bank_accounts(&self) -> Result<String, reqwest::Error> {
        let url = self
            .base_url
            .join("/customer_bank_accounts")
            .expect("Error creating url");

        let result = self.client.get(url).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn get_all_mandate(&self) -> Result<String, reqwest::Error> {
        let url = self.base_url.join("/mandates").expect("Error creating url");

        let result = self.client.get(url).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }

    pub async fn get_customer(&self, id: String) -> Result<String, reqwest::Error> {
        let url = self.base_url.join(&format!("/customers/{id}")).unwrap();

        let result = self.client.get(url).send().await;
        let data = result.ok().unwrap();
        data.text().await
    }
}
