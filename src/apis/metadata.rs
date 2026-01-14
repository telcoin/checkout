use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{CardMetadataFormat, CardMetadataSource, Client, Error};

/// Access the Metadata API.
#[derive(Debug, Clone)]
pub struct CardMetadata<'a> {
    client: &'a Client,
}

impl<'a> CardMetadata<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a single metadata record for the card specified by the Primary
    /// Account Number (PAN), Bank Identification Number (BIN), token, or
    /// instrument supplied.
    ///
    /// [`POST /metadata/card`](https://api-reference.checkout.com/#operation/getCardMetadata)
    pub async fn get_card(
        &self,
        source: CardMetadataSource,
        format: Option<CardMetadataFormat>,
    ) -> Result<CardMetadataResponse, Error> {
        let body = CardMetadataRequest { source, format };
        let url = format!("{}/metadata/card", self.client.environment.api_url());

        self.client
            .send_post_request("vault:card-metadata", &url, &body)
            .await
    }
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
