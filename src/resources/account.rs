use client::Client;
use error::Error;
use params::{List, Metadata, Timestamp};
use resources::BankAccount;
use serde_json as json;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DeclineChargeDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PayoutScheduleDetails {
    pub delay_days: u64,
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TOSAcceptanceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// The set of parameters that can be used when creating an account for users.
///
/// For more details see https://stripe.com/docs/api#create_account.
#[derive(Default, Serialize)]
pub struct AccountParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>, // (country the account holder resides in)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>, // (required if account type is standard)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub account_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account: Option<&'a str>, // (required if account type is standard)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance : Option<TOSAcceptanceDetails>, // (required if account type is standard)
}

/// The resource representing a Stripe account.
///
/// For more details see https://stripe.com/docs/api#account.
#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub business_name: Option<String>,
    pub business_url: Option<String>,
    pub charges_enabled: bool,
    pub country: String,
    pub debit_negative_balances: Option<bool>,
    pub decline_charge_on: Option<DeclineChargeDetails>,
    pub default_currency: String,
    pub details_submitted: bool,
    pub display_name: Option<String>,
    pub email: String,
    pub external_accounts: List<BankAccount>,
    pub legal_entity: Option<json::Value>,
    pub metadata: Metadata,
    pub payout_schedule: Option<PayoutScheduleDetails>,
    pub payout_statement_descriptor: Option<String>,
    pub payouts_enabled: bool,
    pub product_description: Option<String>,
    pub statement_descriptor: String,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub timezone: String,
    pub tos_acceptance: Option<TOSAcceptanceDetails>, // (who accepted Stripe's terms of service)
    #[serde(rename = "type")]
    pub account_type: Option<String>, // (Stripe, Custom, or Express)
    pub verification: Option<json::Value>,
}

impl Account {
    /// Creates a new account.
    ///
    /// For more details see https://stripe.com/docs/api#create_account.
    pub fn create(client: &Client, params: AccountParams) -> Result<Account, Error> {
        client.post("/accounts", params)
    }

    /// Retrieves the details of a account.
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_account.
    pub fn retrieve(client: &Client, account_id: &str) -> Result<Account, Error> {
        client.get(&format!("/accounts/{}", account_id))
    }

    /// Updates a account's properties.
    ///
    /// For more details see https://stripe.com/docs/api#update_account.
    pub fn update(client: &Client, account_id: &str, params: AccountParams) -> Result<Account, Error> {
        client.post(&format!("/accounts/{}", account_id), params)
    }

}
