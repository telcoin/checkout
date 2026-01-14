use crate::models::metadata::{CardMetadataFormat, CardMetadataSource};
use crate::{Client, Error};

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

use crate::models::metadata::{CardMetadataRequest, CardMetadataResponse};
