use client::Client;
use error::Error;
use params::{List, Metadata, Timestamp};
use resources::Currency;

/// The set of parameters that can be used when creating a transfer.
///
/// For more details see https://stripe.com/docs/api/transfer_reversals/create
#[derive(Default, Serialize)]
pub struct TransferReversalParams {
    pub id: String,
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

/// The resource representing a Stripe transfer reversal.
///
/// For more details see https://stripe.com/docs/api#transfer_reversal_object.
#[derive(Debug, Deserialize)]
pub struct TransferReversal {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub balance_transaction: Option<String>,
    pub created: Timestamp,
    pub currency: Currency,
    pub metadata: Metadata,
    pub transfer: String,
}

/// The set of parameters that can be used when creating or updating a transfer.
///
/// For more details see https://stripe.com/docs/api/transfer/create and https://stripe.com/docs/api/transfer/update.
#[derive(Default, Serialize)]
pub struct TransferParams {
    pub amount: u64,
    pub currency: String,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// The resource representing a Stripe transfer.
///
/// For more details see https://stripe.com/docs/api#transfer_object.
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub object: String,
    pub amount: u64,
    pub amount_reversed: u64,
    pub balance_transaction: String,
    pub created: Timestamp,
    pub currency: Currency,
    pub description: Option<String>,
    pub destination: String,
    pub destination_payment: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub reversals: List<TransferReversal>,
    pub reversed: bool,
    pub source_transaction: Option<String>,
    pub source_type: String,
    pub transfer_group: Option<String>,
}

impl Transfer {
    /// Creates a new transfer.
    ///
    /// For more details see https://stripe.com/docs/api/transfer/create.
    pub fn create(client: &Client, params: TransferParams) -> Result<Transfer, Error> {
        client.post("/transfers", params)
    }

    /// Reverses a transfer.
    ///
    /// For more details see https://stripe.com/docs/api/transfer_reversals/create.
    pub fn reverse(
        client: &Client,
        transfer_id: &str,
        params: TransferReversalParams,
    ) -> Result<TransferReversal, Error> {
        client.post(&format!("/transfers/{}/reversals", transfer_id), params)
    }
}
