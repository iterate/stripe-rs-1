use params::Metadata;
use resources::Currency;

/// The set of parameters that can be used when creating a bank account for users.
///
/// For more details see https://stripe.com/docs/api#account_create_bank_account.
#[derive(Default, Serialize)]
pub struct BankAccountParams<'a> {
    pub object: String,  // (bank_account)
    pub country: String, // (country the account holder resides in)
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>, // (required if account type is standard)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<&'static str>, // (individual, company)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'static str>,
    pub account_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// The resource representing a Stripe bank account.
///
/// For more details see https://stripe.com/docs/api#customer_bank_account_object.
#[derive(Debug, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub object: String,
    pub account: String,
    pub account_holder_name: String,
    pub account_holder_type: String, // (individual or company)
    pub bank_name: String,
    pub country: String,
    pub currency: Currency,
    pub default_for_currency: bool,
    pub fingerprint: String,
    pub last4: String,
    pub metadata: Metadata,
    pub routing_number: String,
    pub status: String, // (new, validated, verified, verification_failed, errored)
}
