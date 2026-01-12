use crate::{Client, CreatePaymentSessionRequest, CreatePaymentSessionResponse, Error};
use reqwest::StatusCode;

/// Access the Flows API.
#[derive(Debug, Clone)]
pub struct Flows<'a> {
    client: &'a Client,
}

impl<'a> Flows<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a payment session for the Flow integration.
    ///
    /// [`POST /payment-sessions`](https://api-reference.checkout.com/#operation/CreatePaymentSession)
    pub async fn create_payment_session(
        &self,
        request: &CreatePaymentSessionRequest,
    ) -> Result<CreatePaymentSessionResponse, Error> {
        let url = format!("{}/payment-sessions", self.client.environment.api_url());
        let response = self
            .client
            .send_post_request_2("payment-sessions", &url, request)
            .await?;

        let status = response.status();
        match status {
            StatusCode::CREATED => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let body = response.json().await?;
                Err(Error::InvalidData(body))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(Error::TooManyRequests),
            code => {
                let body = response.text().await?;
                Err(Error::Unknown(code, body))
            }
        }
    }
}
