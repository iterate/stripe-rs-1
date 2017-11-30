use std::collections::HashMap;

use serde_json as json;

use error::Error;
use client::Client;
use params::{List, Timestamp};
use resources::{Currency, Source};

#[derive(Debug, Deserialize)]
pub struct FeeDetails {
    pub amount: u64,
    pub application: String,
    pub currency: Currency,
    pub description: String,
    #[serde(rename = "type")]
    pub fee_type: String, // (application_fee, stripe_fee, or tax)
}

/// The resource representing a Stripe account balance.
///
/// For more details see https://stripe.com/docs/api#balance_object.
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub object: String,
    pub available: Vec<BalanceCurrency>,
    pub connect_reserved: Option<Vec<json::Value>>,
    pub livemode: bool,
    pub pending: Vec<BalanceCurrency>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceCurrency {
    pub currency: String,
    pub amount: u64,
    pub source_types: HashMap<String, u64>,
}

/// The resource representing a Stripe balance transaction.
///
/// For more details see https://stripe.com/docs/api#balance_transaction_object.
#[derive(Debug, Deserialize)]
pub struct BalanceTransaction {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub available_on: Timestamp,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: String,
    pub fee: u64,
    pub fee_details: List<FeeDetails>,
    pub net: u64,
    pub source: Source,
    pub status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
}

impl Balance {
    /// Retrieves the balance of the current account
    ///
    /// For more details see https://stripe.com/docs/api#retrieve_balance.
    pub fn retrieve(client: &Client) -> Result<Balance, Error> {
        client.get(&format!("/balance"))
    }

}
