use error::Error;
use client::Client;
use resources::{Address, Card, Currency};
use params::{Metadata, Timestamp};

#[derive(Serialize)]
pub struct OwnerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}

#[derive(Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Default, Serialize)]
pub struct SourceParams<'a> {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<&'a str>, // (redirect, receiver, code_verification, none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<OwnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<RedirectParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<&'a str>, // (reusable, single-use)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum Source {
    // BitcoinReceiver(...),
    #[serde(rename = "card")]
    Card(Card),
    #[serde(rename = "source")]
    Source(SourceType),
}

#[derive(Debug, Deserialize)]
pub struct SourceAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SourceOwner {
    pub address: SourceAddress,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub verified_address: Option<String>,
    pub verified_email: Option<String>,
    pub verified_name: Option<String>,
    pub verified_phone: Option<String>,
}

// This is not the same as `Card`, since the latter has the required field `id`
#[derive(Debug, Deserialize)]
pub struct SourceCard {
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub brand: String, // (Visa, American Express, MasterCard, Discover, JCB, Diners Club, or Unknown)
    pub country: String, // eg. "US"
    pub cvc_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: String, // (credit, debit, prepaid, unknown)
    pub last4: String,
}


#[derive(Debug, Deserialize)]
pub struct SourceType {
    pub id: String,
    pub amount: u64,
    pub client_secret: Option<String>,
    pub created: Timestamp,
    pub currency: Option<String>,
    pub flow: String,
    pub livemode: bool,
    pub metadata: Metadata,
    pub owner: SourceOwner,
    pub statement_descriptor: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub source_type: String,
    pub usage: String,
    pub card: Option<SourceCard>,
}

impl Source {
    pub fn create(client: &Client, params: SourceParams) -> Result<Source, Error> {
        client.post("/sources", params)
    }

    pub fn get(client: &Client, source_id: &str) -> Result<Source, Error> {
        client.get(&format!("/sources/{}", source_id))
    }

    pub fn update(client: &Client, source_id: &str, params: SourceParams) -> Result<Source, Error> {
        client.post(&format!("/source/{}", source_id), params)
    }
}
