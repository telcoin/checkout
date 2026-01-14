use bon::Builder;
use serde::{Deserialize, Serialize};

/// The source to get card metadata
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[expect(missing_docs)]
pub enum CardMetadataSource {
    #[serde(rename = "card")]
    Card {
        /// The Primary Account Number
        number: String,
    },

    #[serde(rename = "bin")]
    Bin {
        /// The issuer's Bank Identification Number
        bin: String,
    },

    #[serde(rename = "token")]
    Token {
        /// The Checkout.com unique token that was generated when the card's
        /// details were tokenized
        token: String,
    },

    #[serde(rename = "id")]
    Id {
        /// The unique ID for the payment instrument that was created using the
        /// card's details
        id: String,
    },
}

/// Returns a single metadata record for the card specified by the Primary
/// Account Number (PAN), Bank Identification Number (BIN), token, or instrument
/// supplied.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CardMetadataFormat {
    /// A basic response will only include standard metadata
    #[serde(rename = "basic")]
    Basic,

    /// A `card_payouts` formatted response will also include fields specific to
    /// card payouts.
    #[serde(rename = "card_payouts")]
    CardPayouts,
}

/// Request card metadata
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct CardMetadataRequest {
    /// The source object
    pub source: CardMetadataSource,

    /// The format to provide the output in.
    ///
    /// Default is "basic"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CardMetadataFormat>,
}

/// Response with card metadata
#[derive(Deserialize, Debug, Clone)]
pub struct CardMetadataResponse {
    /// The issuer's Bank Identification Number (BIN)
    pub bin: String,

    /// The global card scheme. For example, `american_express`,
    /// `cartes_bancaires`, `diners_club_international`, `discover`,
    /// `jcb`, `mastercard`, or `visa`.
    pub scheme: String,

    /// The card type: `CREDIT`, `DEBIT`, `PREPAID`, `CHARGE`, or `DEFERRED DEBIT`
    pub card_type: Option<String>,

    /// The card category: `CONSUMER` or `COMMERCIAL`
    pub card_category: Option<String>,

    /// The card issuer
    pub issuer: Option<String>,

    /// The card issuer's country, as an ISO-2 code
    pub issuer_country: Option<String>,
    // and more
}
