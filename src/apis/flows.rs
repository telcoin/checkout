use crate::{Client, Error};

use crate::models::flows::{CreatePaymentSessionRequest, CreatePaymentSessionResponse};

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
    /// [`POST /payment-sessions`](https://api-reference.checkout.com/#operation/createPaymentSession)
    pub async fn create_payment_session(
        &self,
        request: &CreatePaymentSessionRequest,
    ) -> Result<CreatePaymentSessionResponse, Error> {
        let url = format!("{}/payment-sessions", self.client.environment.api_url());
        self.client
            .send_post_request("payment-sessions", &url, request)
            .await
    }
}
